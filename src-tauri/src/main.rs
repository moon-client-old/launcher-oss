// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::api::moon::auth::AuthenticationEndpointData;
use crate::gui::LauncherState;
use crate::proprietary::{SerialProvideError, PROPRIETARY_LIBRARY};
use crate::storage::StorageType;
use serde::Serialize;
use tauri::async_runtime::Mutex;

mod api;
mod gui;
mod proprietary;
mod storage;

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(LauncherState {
            serial: "".to_string(),
            session_token: "".to_string(),
            cached_login_data: None,
        }))
        .invoke_handler(tauri::generate_handler![
            gui::login::login,
            gui::login::load_login_settings,
            gui::get_max_available_memory,
            gui::load_serial
        ])
        .run(tauri::generate_context!())
        .expect("error while running gui application");
}
