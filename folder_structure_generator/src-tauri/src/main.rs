// 這是入口文件，進行應用初始化和Tauri命令設定。

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod libs; // 引入lib模組

use libs::folder_structure::{generate_local_documents_structure, generate_folder_structure};
use tauri::{Builder, generate_handler};

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![
            generate_local_documents_structure,
            generate_folder_structure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}