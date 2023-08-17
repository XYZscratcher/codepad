// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs::{File,self};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save(code: &str,path:&str) {
    fs::write(path, code);
}
#[tauri::command]
fn read_file(path:&str)->String{
    fs::read_to_string(path).unwrap()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        //.plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![save,read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
