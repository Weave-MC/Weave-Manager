#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::OsStr;
use std::sync::Mutex;
use serde::Serialize;

use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, System, SystemExt};
use tauri::State;

#[derive(Serialize)]
enum ClientType {
    LunarClient
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

            if !proc.cmd().iter().any(|arg| arg.starts_with("net.minecraft")) {
                return None
            }

            let client = if proc.cmd().iter().any(|arg| arg.contains("lunar")) {
                ClientType::LunarClient
            } else {
                return None
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
