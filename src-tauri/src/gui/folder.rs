use serde::{Deserialize, Serialize};

#[tauri::command]
pub async fn open_directory_type(directory: DirectoryType) {
    let folder_to_open = match directory {
        DirectoryType::Minecraft => crate::storage::location::MINECRAFT_WORKING_DIRECTORY.clone(),
        DirectoryType::Settings => crate::storage::location::MOON_WORKING_DIRECTORY.clone(),
    };
    open::that(folder_to_open).expect("Unable to open directory");
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DirectoryType {
    Minecraft,
    Settings,
}
