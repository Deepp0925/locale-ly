// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn send_decoupled() -> HashMap<String, String> {
    println!("send_decoupled");
    todo!()
}

fn main() {
    // TODO handle properly the window size mechanism
    //
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, send_decoupled])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
