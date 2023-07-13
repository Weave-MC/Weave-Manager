#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    launch_times: [u32; 10],
    time_played: u64,
    average_launch_time: f32,
}

#[derive(Clone, Serialize)]
struct ConsolePayload {
    line: String,
    pid: u32
}

fn get_weave_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut home = home_dir().ok_or("Home directory not found")?;
    home.push(".weave");
    Ok(home)
}

fn get_weave_logs_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let weave_dir = get_weave_directory()?;
    let mut logs_dir = weave_dir.clone();
    logs_dir.push("logs");

    if !logs_dir.exists() {
        fs::create_dir_all(&logs_dir)?;
    }

    Ok(logs_dir)
}

fn get_weave_loader_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let weave_dir = get_weave_directory()?;
    let mut loader_path = weave_dir.clone();
    loader_path.push("loader.jar");

    if !loader_path.exists() {
        return Err("Weave-Loader JAR file (~/.weave/loader.jar) not found".into());
    }

    Ok(loader_path)
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn std::error::Error>> {
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
fn check_loader_integrity(sum_to_check: String) -> bool {
    match get_weave_loader_path() {
        Ok(weave_loader) => {
            if let Ok(input) = File::open(&weave_loader) {
                let reader = BufReader::new(input);
                if let Ok(digest) = sha256_digest(reader) {
                    let checksum = HEXUPPER.encode(digest.as_ref());
                    return sum_to_check == checksum
                }
            }
        }
        Err(err) => {
            // show error modal
        }
    }

    false
}

#[tauri::command]
fn read_mod_config(path: String) -> Option<ModConfig> {
    let f = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(f).unwrap();
    let conf = match archive.by_name("weave.mod.json") {
        Ok(conf) => conf,
        Err(ZipError::FileNotFound) => return None,
        Err(e) => panic!("{:?}", e)
    };
    Some(serde_json::from_reader(conf).unwrap())
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

            let weave_attached = if proc.cmd().iter().any(|arg| arg.contains("Weave-Loader")) {
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
fn relaunch_with_weave(cwd: String, mut cmd_line: Vec<String>, app_state: State<AppState>, app: tauri::AppHandle) {
    if let Ok(weave_loader_path) = get_weave_loader_path() {
        let java_agent = String::from("-javaagent:") + &weave_loader_path.to_str().unwrap();
        cmd_line.insert(1, java_agent);

        // piped outputs
        let (reader, writer) = os_pipe::pipe().expect("Failed to create pipe");

        let child = Command::new(&cmd_line[0])
            .current_dir(Path::new(&cwd))
            .stderr(writer.try_clone().expect("Failed to clone pipe writer"))
            .stdout(writer)
            .args(&cmd_line[1..])
            .spawn().expect("Failed to relaunch with Weave");

        app_state.selected_process.compare_and_swap(0, child.id(), Ordering::Relaxed);

        let selected_process = Arc::clone(&app_state.selected_process);
        std::thread::spawn(move || {
            let log_name = Local::now().format("%Y-%m-%d-%H%M%S.log").to_string();
            if let Ok(logs_path) = get_weave_logs_path() {
                let mut log_file = File::create(logs_path.join(log_name)).expect("Failed to create log file");
                let buf_reader = BufReader::new(reader);

                for line in buf_reader.lines().filter_map(|l| l.ok()) {
                    write!(log_file, "{}\n", line).expect("Failed to write output to log file");

                    if selected_process.load(Ordering::Relaxed) == child.id() {
                        app.emit_all("console_output", ConsolePayload {
                            line,
                            pid: child.id()
                        }).expect("Failed to emit console log to renderer");
                    }
                }
            }
        });
    }
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
fn get_analytics() -> Analytics {
    let analytics_file = get_weave_directory().unwrap().join("analytics.json");

    if let Ok(file_content) = fs::read_to_string(analytics_file) {
        if let Ok(analytics) = serde_json::from_str::<Analytics>(&file_content) {
            return Analytics {
                launch_times: analytics.launch_times,
                time_played: analytics.time_played,
                average_launch_time: analytics.average_launch_time
            }
        }
    }

    Analytics {
        launch_times: [0; 10],
        time_played: 0,
        average_launch_time: 0.0
    }
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