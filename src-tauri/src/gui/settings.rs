use crate::gui::LauncherState;
use crate::storage::types::GameSettingData;
use crate::storage::{StorageError, StorageType};
use tokio::sync::Mutex;

/// Loads the game settings from the working directory
#[tauri::command]
pub async fn load_game_settings(
    state: tauri::State<'_, Mutex<LauncherState>>,
) -> Result<GameSettingData, StorageError> {
    let mut state = state.lock().await;

    // Load data if it is not present in state cache yet
    return if let None = state.cached_game_state {
        let loaded_data = crate::storage::load_storage_data(
            StorageType::GameSettings,
            GameSettingData { memory: 2048 },
        )?;

        state.cached_game_state = Some(loaded_data);
        Ok(loaded_data)
    } else {
        Ok(state.cached_game_state.unwrap())
    };
}

/// Saves the game settings to the working directory
#[tauri::command]
pub async fn save_game_settings(
    state: tauri::State<'_, Mutex<LauncherState>>,
    memory: i64,
) -> Result<(), StorageError> {
    let mut state = state.lock().await;
    let game_settings = GameSettingData { memory };

    // Save the data and store it in the launcher state
    match crate::storage::save_storage_data(StorageType::GameSettings, game_settings) {
        Ok(_) => {}
        Err(error) => {
            return Err(error);
        }
    };
    state.cached_game_state = Some(game_settings);
    Ok(())
}
