#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Read;
use std::process::{self, Stdio};
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use tauri::Manager;
use tauri::{RunEvent,WindowEvent};
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};

use tungstenite::{Message, connect};

/// Stores all stdout contents from the backend process
static BACKEND_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| { Mutex::new(String::new()) });

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_backend_logs() -> String {
    BACKEND_OUTPUT.lock().unwrap().clone()
}

fn main() {
    let tray_menu = {
        let open = CustomMenuItem::new("open".to_string(), "Open");
        let quit = CustomMenuItem::new("quit".to_string(), "Quit");
        SystemTrayMenu::new()
            .add_item(open)
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(quit)
    };
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    let backend: Arc<Mutex<Option<process::Child>>> = Arc::new(Mutex::new(None));

    let backend2 = backend.clone();
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                    match connect("ws://localhost:9901") {
                        Ok((mut socket, _)) => {
                            socket.write_message(Message::Text(r#"{ "subject": "shutdown" }"#.into())).unwrap();
                            socket.close(None).unwrap();
                            println!("Waiting for backend to exit...");
                            backend.lock().unwrap().take().unwrap().wait().unwrap();
                        },
                        Err(_) => {
                            // Can't send shutdown message, so just force kill the backend
                            backend.lock().unwrap().take().unwrap().kill().unwrap();
                        }
                    }
                    app.exit(0);
                }
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
              }
            }
            _ => {}
        })
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
            if let WindowEvent::CloseRequested{ api, .. } = ev.event() {
                api.prevent_close();
                ev.window().hide().unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![get_backend_logs])
        .build(tauri::generate_context!())
        .expect("Error building tauri app")
        .run(|_app, event| match event {
            RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
            _ => (),
        });

}
