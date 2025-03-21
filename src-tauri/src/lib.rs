use arboard::Clipboard;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn watch_clipboard(app: AppHandle) {
    let mut clipboard = Clipboard::new().expect("无法初始化剪贴板");
    let mut last_clipboard_content = String::new();
    thread::spawn(move || loop {
        if let Ok(content) = clipboard.get_text() {
            if content != last_clipboard_content {
                app.emit("clipboard_update", content.clone()).unwrap();
                last_clipboard_content = content;
            }
        }
        thread::sleep(Duration::from_millis(500));
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            watch_clipboard(handle);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
