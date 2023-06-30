#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};
use tauri::{App, Manager, Wry};

#[derive(Clone, serde::Serialize)]
struct Minecraft<'a> {
  cwd: &'a Path,
  args: &'a [String],
  pid: u32
}

fn process_scan_loop(app: &App<Wry>) {
  let handle = app.handle();

  thread::spawn(move || {
    let mut sys = System::new();

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

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      process_scan_loop(app);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
