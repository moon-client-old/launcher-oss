use sys_info::MemInfo;
use tauri::async_runtime::Mutex;

use crate::api::moon::auth::{authenticate, AuthenticationError, AuthenticationResponseData};
use crate::storage::types::LoginSettingData;
use crate::storage::StorageType;

pub mod login;

/// Contains things required multiple times throughout the runtime process
/// This struct is managed by gui and can be used in every gui command
#[derive(Clone)]
pub struct LauncherState {
    pub serial: String,
    pub session_token: String,
    pub cached_login_data: Option<LoginSettingData>,
}

#[tauri::command]
pub async fn load_serial(state: tauri::State<'_, Mutex<LauncherState>>) -> Result<(), ()> {
    let mut guard = state.lock().await;
    let serial = unsafe {
        match crate::proprietary::fetch_serial() {
            Ok(serial) => serial,
            _ => return Err(()),
        }
    };
    guard.serial = serial;
    Ok(())
}

#[tauri::command]
pub async fn get_max_available_memory() -> u64 {
    let info = match sys_info::mem_info() {
        Ok(info) => info,
        Err(_) => return 0,
    };
    info.total
}
