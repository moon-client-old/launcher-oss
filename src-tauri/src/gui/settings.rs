use crate::gui::LauncherState;
use crate::storage::types::{GameSettingData, VersionSelectionData, VersionSettingData};
use crate::storage::{StorageError, StorageType};
use tokio::sync::Mutex;

/// Loads the game settings from the working directory
#[tauri::command]
pub async fn load_game_settings(
    state: tauri::State<'_, Mutex<LauncherState>>,
) -> Result<GameSettingData, StorageError> {
    let mut state = state.lock().await;

    // Load data if it is not present in state cache yet
    match state.cached_game_state {
        Some(data) => Ok(data),
        None => {
            let loaded_data = crate::storage::load_storage_data(
                StorageType::GameSettings,
                GameSettingData { memory: 2048 },
            )?;

            state.cached_game_state = Some(loaded_data);
            Ok(loaded_data)
        }
    }
}

/// Loads the selection settings from the working directory
#[tauri::command]
pub async fn load_selection_settings(
    state: tauri::State<'_, Mutex<LauncherState>>,
) -> Result<VersionSettingData, StorageError> {
    let mut state = state.lock().await;

    // Load data if it is not present in state cache yet
    match state.cached_selection_state {
        Some(ref data) => Ok(data.clone()),
        None => {
            let loaded_data = crate::storage::load_storage_data(
                StorageType::VersionSettings,
                VersionSettingData {
                    selections: Vec::new(),
                },
            )?;

            state.cached_selection_state = Some(loaded_data.clone());
            Ok(loaded_data)
        }
    }
}

/// Loads the selection settings from the working directory
#[tauri::command]
pub async fn load_selection_settings_for(
    state: tauri::State<'_, Mutex<LauncherState>>,
    channel: String,
) -> Result<VersionSelectionData, StorageError> {
    let selection_settings = load_selection_settings(state)
        .await
        .unwrap_or(VersionSettingData {
            selections: Vec::new(),
        });

    // Load selection data or fall-back to default
    let selection_data = selection_settings
        .selections
        .into_iter()
        .find(|selection| selection.channel == channel)
        .unwrap_or(VersionSelectionData {
            channel,
            preferred_version: "".to_string(),
            requires_latest: true,
        });

    Ok(selection_data)
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

/// Saves the game settings to the working directory
#[tauri::command]
pub async fn save_selection_settings_for(
    state: tauri::State<'_, Mutex<LauncherState>>,
    channel: String,
    version: String,
    always_latest: bool,
) -> Result<(), StorageError> {
    let mut state = state.lock().await;
    let current_selection = state
        .cached_selection_state
        .clone()
        .unwrap_or(VersionSettingData {
            selections: Vec::new(),
        });

    // Update the selection data (don't ask what this is, but i call it the definition of shitty code)
    let selections = current_selection.selections;
    let mut new_selection_data = Vec::new();
    let mut found_existing_channel = false;
    for existing_selection in selections {
        if existing_selection.channel == channel {
            new_selection_data.push(VersionSelectionData {
                channel: channel.clone(),
                preferred_version: version.clone(),
                requires_latest: always_latest,
            });
            found_existing_channel = true;
        } else {
            new_selection_data.push(VersionSelectionData {
                channel: existing_selection.channel,
                preferred_version: existing_selection.preferred_version,
                requires_latest: existing_selection.requires_latest,
            });
        }
    }

    // Insert channel if not existing
    if !found_existing_channel {
        new_selection_data.push(VersionSelectionData {
            channel: channel.clone(),
            preferred_version: version.clone(),
            requires_latest: always_latest,
        });
    }

    // Save the data and store it in the launcher state
    state.cached_selection_state = Some(VersionSettingData {
        selections: new_selection_data.clone(),
    });
    match crate::storage::save_storage_data(
        StorageType::VersionSettings,
        VersionSettingData {
            selections: new_selection_data,
        },
    ) {
        Ok(_) => {}
        Err(error) => {
            return Err(error);
        }
    };

    Ok(())
}
