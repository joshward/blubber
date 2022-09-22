#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn get_app_location(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path_resolver().app_dir().expect("No app directory defined for OS");

    Ok(app_dir.to_str().unwrap().into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_app_location])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
