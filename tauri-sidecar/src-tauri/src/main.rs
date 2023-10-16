// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
  api::process::{Command, CommandEvent}
};

fn main() {
  tauri::Builder::default()
  .setup(|_app| {
    tauri::async_runtime::spawn(async move {
      let (mut rx, _child) = Command::new_sidecar("main")
        .expect("failed to setup `main` sidecar")
        .spawn()
        .expect("Failed to spawn packaged node");

      while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
          println!("{}", format!("'{}'", line));
        }
      }
    });

    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
