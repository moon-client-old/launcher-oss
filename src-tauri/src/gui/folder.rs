use serde::{Deserialize, Serialize};

/// Opens a given [DirectoryType] inside the operating systems file explorer
#[tauri::command]
pub async fn open_directory_type(directory: DirectoryType) {
    let folder_to_open = match directory {
        DirectoryType::Minecraft => crate::storage::location::MINECRAFT_WORKING_DIRECTORY.clone(),
        DirectoryType::Settings => crate::storage::location::MOON_WORKING_DIRECTORY.clone(),
    };
    open::that(folder_to_open).expect("Unable to open directory");
}

/// All different directory types which can be opened through tauri commands
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DirectoryType {
    Minecraft,
    Settings,
}
