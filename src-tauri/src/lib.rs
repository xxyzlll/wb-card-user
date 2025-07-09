// src-tauri/src/lib.rs

// 导入API模块
mod api;

// 使用API模块中的函数
use api::{fetch_fans, send_direct_message, fetch_user_timeline};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_fans, 
            send_direct_message,
            fetch_user_timeline
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}