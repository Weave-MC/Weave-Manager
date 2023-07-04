#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::OsStr;
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;
use std::env;
use serde::Serialize;

use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, System, SystemExt};
use tauri::State;

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
    client_type: ClientType
}

#[tauri::command]
fn fetch_minecraft_instances(system: State<Mutex<System>>) -> Vec<MinecraftInstance> {
    let mut system = system.lock().unwrap();
    system.refresh_processes_specifics(ProcessRefreshKind::new());

    system.processes().values()
        .filter_map(|proc| {
            if !matches!(proc.exe().file_name().and_then(OsStr::to_str), Some("javaw.exe" | "java")) {
                return None
            }

            if !proc.cmd().iter().any(|arg| arg.contains(".minecraft")) {
                return None
            }

            let client = if proc.cmd().iter().any(|arg| arg.contains("lunar")) {
                ClientType::LunarClient
            } else if proc.cmd().iter().any(|arg| arg.contains("minecraftforge")) {
                ClientType::Forge
            } else {
                ClientType::Vanilla
            };

            Some(MinecraftInstance {
                pid: proc.pid().as_u32(),
                cmd: proc.cmd().to_owned(),
                cwd: proc.cwd().to_string_lossy().to_string(),
                version: proc.cmd().iter().skip_while(|&arg| arg != "--version").nth(1)?.clone(),
                start_time: proc.start_time(),
                client_type: client
            })
        })
        .collect()
}

#[tauri::command]
fn relaunch_with_weave(cwd: String, cmd_line: Vec<String>) {
    let weave_loader_location = get_weave_loader_path();

    let mut updated_cmd = cmd_line;

    if !weave_loader_location.is_none() {
        let java_agent = String::from("-javaagent:") + &weave_loader_location.unwrap().as_path().to_str().unwrap();
        updated_cmd.insert(1, java_agent);

        let _child = Command::new(&updated_cmd[0])
            .current_dir(Path::new(&cwd))
            .args(&updated_cmd[1..])
            .spawn()
            .expect("Failed to relaunch with Weave");
    }
}

fn get_weave_loader_path() -> Option<PathBuf> {
    match env::home_dir() {
        Some(path) => {
            let weave_dir = path.join(".weave");

            if weave_dir.is_dir() {
                for entry in fs::read_dir(weave_dir).ok()? {
                    if let Ok(entry) = entry {
                        if entry.file_name().to_string_lossy().starts_with("Weave-Loader") {
                            return Some(entry.path())
                        }
                    }
                }
            }
        },
        None => eprintln!("Impossible to get your home dir"),
    }

    None
}

#[tauri::command]
fn kill_pid(pid: u32, system: State<Mutex<System>>) -> bool {
    system.lock().unwrap().process(Pid::from_u32(pid)).is_some_and(|p| p.kill())
}

#[tauri::command]
fn get_memory_usage(system: State<Mutex<System>>) -> Vec<String> {
    let mut sys = system.lock().unwrap();
    sys.refresh_all();

    let total = sys.total_memory();

    let process = sys.process(sysinfo::get_current_pid().unwrap()).unwrap();
    let used = process.memory();

    vec![used.to_string(), total.to_string()]
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs_watch::init())
        .manage(Mutex::new(System::new()))
        .invoke_handler(tauri::generate_handler![fetch_minecraft_instances, kill_pid, get_memory_usage, relaunch_with_weave])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
