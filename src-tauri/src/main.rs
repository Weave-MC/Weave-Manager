#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;

use std::collections::HashMap;
use error::Result;

use std::ffi::OsStr;
use std::sync::{Mutex, Arc};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::fs::{File, read_dir, rename};
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

#[derive(Clone, Serialize, Deserialize)]
enum ClientType {
    Lunar,
    Forge,
    Labymod,
    Vanilla,
    Badlion,
    Feather
}
#[derive(Serialize)]
struct MinecraftProcess {
    pid: u32,
    start_time: u64,
    info: MinecraftInfo,
    weave_attached: bool
}
#[derive(Serialize, Deserialize)]
struct MinecraftInfo {
    client: ClientType,
    version: String,
    cmd: Vec<String>,
    cwd: String
}
#[derive(Serialize, Deserialize)]
struct ModProfile {
    name: String, // names must be unique
    mods: Vec<ModProfileEntry>
}
#[derive(Serialize, Deserialize)]
struct ModProfileEntry {
    config: Option<ModConfig>,
    file_name: String // path is scoped in ~/.weave/mods
}
#[derive(Serialize, Deserialize)]
struct ModConfig {
    name: String,
    version: String,
    description: String,
    authors: Vec<String>
}
impl Default for ModConfig {
    fn default() -> Self {
        ModConfig {
            name: "undefined".to_string(),
            version: "undefined".to_string(),
            description: "undefined".to_string(),
            authors: Vec::new()
        }
    }
}
#[derive(Serialize, Deserialize)]
struct LaunchProfile {
    name: String,
    mc_info: MinecraftInfo,
    mod_profile: Option<ModProfile>
}

#[derive(Deserialize, Serialize)]
struct Analytics {
    launch_times: Vec<u32>,
    time_played: u64,
    average_launch_time: f32,
}

#[derive(Clone, Serialize)]
struct ConsolePayload {
    line: String
}

#[derive(Clone, Serialize)]
struct WeaveProcess {
    log_file: PathBuf,
    client: ClientType,
    pid: u32,
    output: Vec<String>
}

fn get_weave_directory() -> Result<PathBuf> {
    Ok(home_dir().ok_or("Home directory not found")?.join(".weave"))
}

fn get_weave_client_logs_path() -> Result<PathBuf> {
    let log_dir = get_weave_directory()?.join("logs").join("client");
    if !log_dir.exists() {
        fs::create_dir(&log_dir)?;
    }
    Ok(log_dir)
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
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;
    let conf = match archive.by_name("weave.mod.json") {
        Ok(conf) => conf,
        Err(ZipError::FileNotFound) => return Ok(Some(ModConfig::default())),
        Err(e) => Err(e)?
    };
    match serde_json::from_reader(conf) {
        Ok(config) => Ok(Some(config)),
        Err(_) => Ok(Some(ModConfig::default()))
    }
}

#[tauri::command]
fn fetch_minecraft_processes(app_state: State<AppState>) -> Vec<MinecraftProcess> {
    let mut system = app_state.system.lock().unwrap();
    system.refresh_processes_specifics(ProcessRefreshKind::new()); // refresh processes

    system.processes().values()
        .filter_map(|proc| {
            // If there are no java processes, return None
            if !matches!(proc.exe().file_name().and_then(OsStr::to_str), Some("javaw.exe" | "java")) {
                return None
            }

            // Rudimentary check for if the process is Minecraft
            if !proc.cmd().iter().any(|arg| arg.contains("minecraft")) {
                return None
            }

            // Determine the client type via command line arguments
            let mut client_type = ClientType::Vanilla;
            for arg in proc.cmd().iter() {
                if arg.contains("lunar") {
                    client_type = ClientType::Lunar;
                    break;
                } else if arg.contains("forge") {
                    client_type = ClientType::Forge;
                    break;
                } else if arg.contains("labymod") {
                    client_type = ClientType::Labymod;
                    break;
                }
            }

            let weave_attached = proc.cmd().iter().any(|arg| arg.contains("loader.jar") && arg.contains("-javaagent"));

            Some(MinecraftProcess {
                pid: proc.pid().as_u32(),
                start_time: proc.start_time(),
                info: MinecraftInfo {
                    client: client_type,
                    version: proc.cmd().iter().skip_while(|&arg| arg != "--version").nth(1)?.clone(),
                    cmd: proc.cmd().to_owned(),
                    cwd: proc.cwd().to_string_lossy().to_string()
                },
                weave_attached
            })
        }).collect()
}

#[tauri::command]
fn launch(profile: LaunchProfile, app_state: State<AppState>, app: tauri::AppHandle) -> Result<()> {
    let mc = profile.mc_info;
    let weave_loader_path = get_weave_loader_path()?;

    // Insert the weave agent to the command line
    let mut cmd = mc.cmd;
    cmd.retain(|s| !s.starts_with("-Dlog4j"));
    cmd.insert(1, format!("-javaagent:{}", weave_loader_path.to_str().unwrap()));

    // piped outputs
    let (reader, writer) = os_pipe::pipe()?;

    // spawn the process
    let child = Command::new(&cmd[0])
        .current_dir(Path::new(&mc.cwd))
        .stderr(writer.try_clone()?)
        .stdout(writer)
        .args(&cmd[1..])
        .spawn()?;

    // capture these values before moving into the closure
    let log_dir = get_weave_client_logs_path()?;
    let log_name = Local::now().format("%Y-%m-%d-%H%M%S.log").to_string();
    let log_path = log_dir.join(log_name);

    // select the most recent process spawned as the console output
    app_state.selected_process.store(child.id(), Ordering::Relaxed);
    // create a clone that can safely be referenced after move
    let selected_process = Arc::clone(&app_state.selected_process);

    // pipe the output to a file and emit an event containing the line
    std::thread::spawn(move || {
        let mut log_file = File::create(&log_path).expect("Failed to create log file");

        app.emit_all("spawned_weave", WeaveProcess {
            log_file: log_path,
            client: mc.client,
            pid: child.id(),
            output: Vec::new()
        }).expect("Failed to emit spawned_weave event to renderer");

        let buf_reader = BufReader::new(reader);

        for line in buf_reader.lines().filter_map(|l| l.ok()) {
            write!(log_file, "{}\n", line).expect("Failed to write output to log file");

            if selected_process.load(Ordering::Relaxed) == child.id() {
                app.emit_all("console_output", ConsolePayload {
                    line
                }).expect("Failed to emit console_output event to renderer");
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn switch_console_output(pid: u32, app_state: State<AppState>) {
    app_state.selected_process.store(pid, Ordering::Relaxed);
}

#[tauri::command]
fn kill_pid(pid: u32, app_state: State<AppState>) -> bool {
    // app_state.weave_processes.remove(&pid);
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
            fetch_minecraft_processes,
            kill_pid,
            get_memory_usage,
            get_analytics,
            launch,
            read_mod_config,
            switch_console_output,
            check_loader_integrity
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}