use std::thread;
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter,
};
use tauri_plugin_clipboard_manager::ClipboardExt;

mod win;

#[tauri::command]
fn set_window_text(text: &str) {
    win::Win::send_message(text);
}

fn watch_clipboard(app: AppHandle) {
    let mut last_clipboard_content = String::new();
    thread::spawn(move || loop {
        if let Ok(content) = app.clipboard().read_text() {
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
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let handle = app.handle().clone();
            watch_clipboard(handle);

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_window_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
