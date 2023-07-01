#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use tauri::{App, Manager, Wry};
use std::sync::Mutex;

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
        // this is total RSS memory, which will differ from what's shown on Task Manager
        let used = process.memory();
        let percent = (used as f64 / total as f64) * 100.0;
        let percent_string = format!("{:.2}", percent);
        return percent_string
      }
    }
    Err(e) => {
      eprintln!("failed to get current pid: {}", e)
    }
  }

  "32".into()
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      process_scan_loop(app);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_memory_usage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
