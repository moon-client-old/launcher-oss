use crate::api;
use crate::api::moon::auth::{
    authenticate, AuthenticationEndpointData, AuthenticationError, AuthenticationResponseData,
};
use sys_info::{Error, MemInfo};
use tauri::async_runtime::Mutex;

/// Contains things required multiple times throughout the runtime process
/// This struct is managed by gui and can be used in every gui command
#[derive(Clone)]
pub struct LauncherState {
    pub serial: String,
    pub session_token: String,
}

#[tauri::command]
pub async fn get_max_available_memory() -> u64 {
    let info = match sys_info::mem_info() {
        Ok(info) => info,
        Err(_) => return 0,
    };
    info.total
}

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, Mutex<LauncherState>>,
    uid: &str,
) -> Result<AuthenticationResponseData, AuthenticationError> {
    let mut state = state.lock().await;
    let uid_i = uid.parse::<i64>().unwrap_or(0);
    let authentication_data = authenticate(&state, uid_i).await;
    // Update the session token if possible
    if let Ok(data) = &authentication_data {
        state.session_token = data.session_key.clone()
    }
    authentication_data
}
