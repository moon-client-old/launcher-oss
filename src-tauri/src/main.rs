// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::api::moon::auth::AuthenticationEndpointData;
use crate::gui::LauncherState;
use crate::proprietary::{SerialProvideError, PROPRIETARY_LIBRARY};
use tauri::async_runtime::Mutex;

mod api;
mod gui;
mod proprietary;
mod storage;

fn main() {
    let serial = unsafe { proprietary::fetch_serial() };
    tauri::Builder::default()
        .manage(Mutex::new(LauncherState {
            serial: serial.unwrap(),
            session_token: "".to_string(),
        }))
        .invoke_handler(tauri::generate_handler![
            gui::login,
            gui::get_max_available_memory
        ])
        .run(tauri::generate_context!())
        .expect("error while running gui application");
}
