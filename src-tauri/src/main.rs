#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dirs::home_dir;
use std::ffi::OsStr;
use std::sync::Mutex;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::fs;
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
                version: proc.cmd().iter().skip_while(|&arg| arg != "--version").nth(1)?.clone(),
                start_time: proc.start_time(),
                client_type: client
            })
        })
        .collect()
}

#[tauri::command]
fn relaunch_with_weave(cmd: Vec<String>) -> bool {
    let weave_loader_location = get_weave_loader_path();
    let mut updated_cmd = cmd;
    updated_cmd.push("-javaagent:".to_owned() + &weave_loader_location);

    let result = Command::new(&updated_cmd[0])
        .args(&updated_cmd[1..])
        .spawn();

    match result {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Failed to relaunch with Weave: {}", err);
            exit(1);
        }
    }
}

fn get_weave_loader_path() -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    let weave_dir = home_dir.join(".weave");

    if let Ok(entries) = fs::read_dir(&weave_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy();
                if file_name.starts_with("Weave-Loader") {
                    return Some(entry.path());
                }
            }
        }
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
        .manage(Mutex::new(System::new()))
        .invoke_handler(tauri::generate_handler![fetch_minecraft_instances, kill_pid, get_memory_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
