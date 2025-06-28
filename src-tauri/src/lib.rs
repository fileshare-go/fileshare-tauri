use tokio::io;

pub mod fileshare;
mod loader;
mod util;

mod gen {
    include!(concat!(env!("OUT_DIR"), "/fileshare.rs"));
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn upload(path: &str) -> Result<(), String> {
    fileshare::upload(path).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, upload])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
