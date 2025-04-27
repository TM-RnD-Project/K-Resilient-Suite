// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod kr_ibe;

use kr_ibe::api::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            setup_command,
            extract_command,
            encrypt_command,
            decrypt_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}