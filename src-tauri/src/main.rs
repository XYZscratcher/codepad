// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::env::args;
use std::fs::{self, File};
use std::io::ErrorKind;
//use tauri::Manager;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn save(code: &str, path: &str) {
    fs::write(path, code);
}
#[tauri::command]
fn read_file(path: &str) -> Result<String, String> {
    let f = fs::read_to_string(path);
    if f.is_ok() {
        Ok(f.unwrap())
    } else {
        Err(f.unwrap_or_else(|e| match e.kind() {
            ErrorKind::InvalidData => "Invalid data".to_owned(),
            _ => "Other error".to_owned(),
        }))
    }
}
#[tauri::command]
fn get_args()->Vec<String>{
    args().collect()
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        //.plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![save, read_file,get_args])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
