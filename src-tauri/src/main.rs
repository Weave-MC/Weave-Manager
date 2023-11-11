#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
use error::Result;

use std::ffi::OsStr;
use std::sync::{Mutex, Arc};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::File;
use std::sync::atomic::{AtomicU32, Ordering};
use serde::{Serialize, Deserialize};
use serde_json;

use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, System, SystemExt};
use tauri::{Manager, State, SystemTrayEvent};
use tauri::{SystemTray, SystemTrayMenu, CustomMenuItem, SystemTrayMenuItem};
use tauri_plugin_autostart::MacosLauncher;
use zip::result::ZipError;
use zip::ZipArchive;
use chrono::prelude::Local;
use tauri::api::path::home_dir;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};

#[derive(Serialize)]
enum ClientType {
    LunarClient,
    Forge,
    Vanilla
}

#[derive(Serialize)]
struct MinecraftInstance {
    pid: u32,
    cmd: Vec<String>,
    cwd: String,
    version: String,
    start_time: u64,
    client_type: ClientType,
    weave_attached: bool
}

#[derive(Serialize, Deserialize)]
struct ModConfig {
    name: Option<String>,
    author: Option<String>,
    version: Option<String>,
    link: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Analytics {
    launch_times: Vec<u32>,
    time_played: u64,
    average_launch_time: f32,
}

#[derive(Clone, Serialize)]
struct ConsolePayload<'a> {
    line: String,
    file_path: &'a String
}

fn get_weave_directory() -> Result<PathBuf> {
    Ok(home_dir().ok_or("Home directory not found")?.join(".weave"))
}

fn get_weave_logs_path() -> Result<PathBuf> {
    let logs_dir = get_weave_directory()?.join("logs");
    if !logs_dir.exists() {
        fs::create_dir(&logs_dir)?;
    }
    Ok(logs_dir)
}

fn get_weave_loader_path() -> Result<PathBuf> {
    let loader_path = get_weave_directory()?.join("loader.jar");
    if !loader_path.exists() {
        Err("Weave-Loader JAR file (~/.weave/loader.jar) not found")?;
    }
    Ok(loader_path)
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

#[tauri::command]
fn check_loader_integrity(sum_to_check: String) -> Result<bool> {
    let file = File::open(get_weave_loader_path()?)?;
    let digest = sha256_digest(file)?;
    Ok(sum_to_check == HEXUPPER.encode(digest.as_ref()))
}

#[tauri::command]
fn read_mod_config(path: String) -> Result<Option<ModConfig>> {
    let f = File::open(&path)?;
    let mut archive = ZipArchive::new(f)?;
    let conf = match archive.by_name("weave.mod.json") {
        Ok(conf) => conf,
        Err(ZipError::FileNotFound) => return Ok(None),
        Err(e) => Err(e)?
    };
    Ok(serde_json::from_reader(conf)?)
}

#[tauri::command]
fn fetch_minecraft_instances(app_state: State<AppState>) -> Vec<MinecraftInstance> {
    let mut system = app_state.system.lock().unwrap();
    system.refresh_processes_specifics(ProcessRefreshKind::new());

    system.processes().values()
        .filter_map(|proc| {
            if !matches!(proc.exe().file_name().and_then(OsStr::to_str), Some("javaw.exe" | "java")) {
                return None
            }

            if !proc.cmd().iter().any(|arg| arg.contains(".minecraft")) {
                return None
            }

            let client_type = if proc.cmd().iter().any(|arg| arg.contains("lunar")) {
                ClientType::LunarClient
            } else if proc.cmd().iter().any(|arg| arg.contains("minecraftforge")) {
                ClientType::Forge
            } else {
                ClientType::Vanilla
            };

            let weave_attached = if proc.cmd().iter().any(|arg| arg.contains("loader.jar") && arg.contains("-javaagent")) {
                true
            } else {
                false
            };

            Some(MinecraftInstance {
                pid: proc.pid().as_u32(),
                cmd: proc.cmd().to_owned(),
                cwd: proc.cwd().to_string_lossy().to_string(),
                version: proc.cmd().iter().skip_while(|&arg| arg != "--version").nth(1)?.clone(),
                start_time: proc.start_time(),
                client_type,
                weave_attached
            })
        })
        .collect()
}

#[tauri::command]
fn relaunch_with_weave(cwd: String, mut cmd_line: Vec<String>, app_state: State<AppState>, app: tauri::AppHandle) -> Result<()> {
    let weave_loader_path = get_weave_loader_path()?;
    cmd_line.insert(1, format!("-javaagent:{}", weave_loader_path.to_str().unwrap()));

    // piped outputs
    let (reader, writer) = os_pipe::pipe()?;

    let child = Command::new(&cmd_line[0])
        .current_dir(Path::new(&cwd))
        .stderr(writer.try_clone()?)
        .stdout(writer)
        .args(&cmd_line[1..])
        .spawn()?;

    app_state.selected_process.store(child.id(), Ordering::Relaxed);

    let selected_process = Arc::clone(&app_state.selected_process);
    let log_dir = get_weave_logs_path()?;
    let log_name = Local::now().format("%Y-%m-%d-%H%M%S.log").to_string();
    std::thread::spawn(move || {
        let log_path = log_dir.join(log_name);
        let mut log_file = File::create(&log_path).expect("Failed to create log file");
        let buf_reader = BufReader::new(reader);

        for line in buf_reader.lines().filter_map(|l| l.ok()) {
            write!(log_file, "{}\n", line).expect("Failed to write output to log file");

            if selected_process.load(Ordering::Relaxed) == child.id() {
                app.emit_all("console_output", ConsolePayload {
                    line,
                    file_path: &log_path.display().to_string()
                }).expect("Failed to emit console log to renderer");
            }
        }
    });
    Ok(())
}

#[tauri::command]
fn switch_console_output(pid: u32, app_state: State<AppState>) {
    app_state.selected_process.store(pid, Ordering::Relaxed)
}

#[tauri::command]
fn kill_pid(pid: u32, app_state: State<AppState>) -> bool {
    app_state.system.lock().unwrap().process(Pid::from_u32(pid)).is_some_and(|p| p.kill())
}

#[tauri::command]
fn get_memory_usage(app_state: State<AppState>) -> (u64, u64) {
    let mut sys = app_state.system.lock().unwrap();
    sys.refresh_processes_specifics(ProcessRefreshKind::new());

    let total = sys.total_memory();
    let process = sys.process(sysinfo::get_current_pid().unwrap()).unwrap();
    let used = process.memory();

    (used, total)
}

#[tauri::command]
fn get_analytics() -> Result<Analytics> {
    let analytics_file = get_weave_directory()?.join("analytics.json");
    let analytics = serde_json::from_reader(File::open(analytics_file)?)?;
    Ok(analytics)
}

struct AppState {
    system: Mutex<System>,
    selected_process: Arc<AtomicU32>
}

fn main() {
    let app_state = AppState {
        system: Mutex::new(System::new_all()),
        selected_process: Arc::new(0.into())
    };

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "Show"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                app.get_window("main").unwrap().show().unwrap()
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "show" => app.get_window("main").unwrap().show().unwrap(),
                    "quit" => std::process::exit(0),
                    _ => {}
                }
            }
            _ => {}
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            fetch_minecraft_instances,
            kill_pid,
            get_memory_usage,
            get_analytics,
            relaunch_with_weave,
            read_mod_config,
            switch_console_output,
            check_loader_integrity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}