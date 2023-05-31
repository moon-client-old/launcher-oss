use crate::api;
use crate::api::moon::auth::{authenticate, AuthenticationEndpointData};
use tauri::async_runtime::Mutex;

/// Contains things required multiple times throughout the runtime process
/// This struct is managed by gui and can be used in every gui command
#[derive(Clone)]
pub struct LauncherState {
    pub serial: String,
    pub session_token: String,
}

#[tauri::command]
pub async fn greet(
    state: tauri::State<'_, Mutex<LauncherState>>,
    name: &str,
) -> Result<String, ()> {
    let mut state = state.lock().await;
    println!("{}", state.session_token);
    println!("{:?}", authenticate(&state, 2).await);
    state.session_token = "Insane".to_string();
    Ok(name.to_string())
}
