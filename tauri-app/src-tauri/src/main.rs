#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Read;
use std::process::{self, Stdio};
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use tauri::WindowEvent;

/// Stores all stdout contents from the backend process
static BACKEND_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| { Mutex::new(String::new()) });

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_backend_logs() -> String {
    BACKEND_OUTPUT.lock().unwrap().clone()
}

fn main() {
    let backend: Arc<Mutex<Option<process::Child>>> = Arc::new(Mutex::new(None));

    let backend2 = backend.clone();
    tauri::Builder::default()
        .setup(move |app| {
            let backend_path = app.path_resolver()
                .resolve_resource("backend/lumos-rs-x86_64-pc-windows-msvc.exe")
                .expect("Failed to locate backend executable");

            let mut backend_proc = process::Command::new(backend_path)
                .stdout(Stdio::piped())
                .env("NO_COLOR", "true")
                .spawn().expect("Failed to spawn backend process");
            let mut stdout = backend_proc.stdout.take().unwrap();
            std::thread::spawn(move || {
                let mut buf = vec![0u8; 256];
                loop {
                    match stdout.read(&mut buf) {
                        Ok(len) => {
                            let mut accumulated_output = BACKEND_OUTPUT.lock().unwrap();
                            accumulated_output.push_str(&String::from_utf8_lossy(&buf[0..len]));
                        },
                        Err(_) => break,
                    }
                }
            });
            *backend2.lock().unwrap() = Some(backend_proc);
            Ok(())
        })
        .on_window_event(move |ev| {
            if let WindowEvent::CloseRequested{ .. } = ev.event() {
                backend.lock().unwrap().take().unwrap().wait().unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![get_backend_logs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
