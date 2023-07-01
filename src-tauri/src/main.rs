#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::fs;
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use tauri::{App, Manager, Wry};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use serde_json;

lazy_static::lazy_static! {
  static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

#[derive(Clone, serde::Serialize)]
struct Minecraft<'a> {
  cwd: &'a Path,
  args: &'a [String],
  pid: u32
}

fn process_scan_loop(app: &App<Wry>) {
  let handle = app.handle();

  thread::spawn(move || {
    let mut sys = System::new_all();

    loop {
      sys.refresh_processes_specifics(ProcessRefreshKind::new());

      for (pid, proc) in sys.processes() {

        if matches!(proc.exe().file_name().and_then(OsStr::to_str), Some("javaw.exe" | "java"))
          && proc.cmd().iter().any(|arg| arg.starts_with("1.8"))
        {
          _ = handle.emit_all("minecraft-found", Minecraft {
            cwd: proc.cwd(),
            args: proc.cmd(),
            pid: pid.as_u32()
          })
        }
      }

      thread::sleep(Duration::from_secs_f32(1.0));
    }
  });
}

#[tauri::command]
fn get_memory_usage() -> String {
  let mut sys = SYS.lock().unwrap();
  sys.refresh_all();
  let total = sys.total_memory();

  match sysinfo::get_current_pid() {
    Ok(pid) => {
      if let Some(process) = sys.process(pid) {
        // this is total RSS memory, which can differ from what's shown on Task Manager
        let used = process.memory();
        let percent = (used as f64 / total as f64) * 100.0;
        let percent_string = format!("{:.2}", percent);
        return percent_string;
      }
    }
    Err(e) => {
      eprintln!("failed to get current pid: {}", e)
    }
  }

  "N/A".into()
}

#[derive(Debug, Deserialize, Serialize)]
struct Analytics {
  launchTimes: Vec<u32>,
  averageLaunchTime: String,
}

#[tauri::command]
fn get_avg_launch_time() -> String {
  let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
  let file_path = Path::new(&home_dir).join(".weave").join("analytics.json");

  // check if file exists
  let path_buf = PathBuf::from(&file_path);
  if !path_buf.exists() {
    return "N/A".into()
  }

  if let Ok(file_content) = fs::read_to_string(file_path) {
    if let Ok(analytics) = serde_json::from_str::<Analytics>(&file_content) {
      return analytics.averageLaunchTime;
    }
  }

  "N/A".into()
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      process_scan_loop(app);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_memory_usage, get_avg_launch_time])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
