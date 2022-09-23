#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

#[tauri::command]
fn get_app_location(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle
        .path_resolver()
        .app_dir()
        .expect("No app directory defined for OS");

    Ok(app_dir.to_str().unwrap().into())
}

#[tauri::command]
fn open_settings(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle
        .get_window("settings")
        .unwrap()
        .show()
        .map_err(|err| err.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_location, open_settings])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
