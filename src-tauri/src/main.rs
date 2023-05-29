// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::proprietary::{SerialProvideError, PROPRIETARY_LIBRARY};

mod proprietary;
mod storage;


fn main() {
    // unsafe { println!("{:#?}", proprietary::fetch_serial()) }
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
