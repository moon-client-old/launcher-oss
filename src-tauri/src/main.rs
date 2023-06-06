// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::async_runtime::Mutex;

use crate::gui::LauncherState;
use crate::proprietary::PROPRIETARY_LIBRARY;

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
            cached_game_state: None,
        }))
        .invoke_handler(tauri::generate_handler![
            gui::login::login,
            gui::login::load_login_settings,
            gui::get_max_available_memory,
            gui::load_serial,
            gui::settings::load_game_settings,
            gui::settings::save_game_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running gui application");
}
