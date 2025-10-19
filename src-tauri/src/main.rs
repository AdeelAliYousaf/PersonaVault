// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Runtime};
use reqwest::Client;

#[tauri::command]
async fn get_data_from_fastapi() -> Result<String, String> {
    let client = Client::new();

    // Securely call your FastAPI backend
    let res = client
        .get("http://127.0.0.1:8000/")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    Ok(text)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data_from_fastapi])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
