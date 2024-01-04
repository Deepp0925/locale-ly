// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core_m::{AllTranslations, AllTranslationsInner};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::{Manager, State};

// impl Serialize for AllTranslations {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.
//         todo!()
//     }
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_all_transactions(state: State<AllTranslations>) -> AllTranslations {
    let transations = state.inner().clone();
    transations
}

#[tauri::command]
fn print_all_transactions(state: State<AllTranslations>) {
    let transations = state.inner().clone();
    println!("{:?}", transations);
}

fn main() {
    // TODO handle properly the window size mechanism
    //
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AllTranslations {
                inner: Arc::new(RwLock::new(AllTranslationsInner {
                    locales: vec![],
                    translations: HashMap::new(),
                })),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            print_all_transactions,
            get_all_transactions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
