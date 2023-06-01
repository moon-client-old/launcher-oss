use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::storage::location::MOON_WORKING_DIRECTORY;

pub mod location;
pub mod types;

pub trait NamedStorage {
    /// Fetches the file name for a named storage implementation
    fn file_name(&self) -> &'static str;
}

/// All different storage types available (all storages listed are saved in JSON format)
pub enum StorageType {
    Login,
    GameSettings,
    WineSettings, // TODO: Those are for later once Wine is actually implemented
}

/// All errors which can occur upon saving or loading config files
#[derive(Debug, Serialize, Deserialize)]
pub enum StorageError {
    FileOpenError { message: &'static str },
    JsonSerializeError { message: &'static str },
    StorageSaveError { message: &'static str },
    StorageLoadError { message: &'static str },
}

impl NamedStorage for StorageType {
    fn file_name(&self) -> &'static str {
        return match self {
            StorageType::Login => "login",
            StorageType::GameSettings => "game",
            StorageType::WineSettings => "wine",
            _ => "unknown",
        };
    }
}

/// Saves data to a [StorageType] inside the working directory
pub fn save_storage_data<T: Serialize>(
    storage_type: StorageType,
    data: T,
) -> Result<(), StorageError> {
    let serialized =
        serde_json::to_string(&data).map_err(|_| StorageError::JsonSerializeError {
            message: "Failed to serialize given storage data",
        })?;

    let mut storage_path = MOON_WORKING_DIRECTORY.clone();
    storage_path.push(format!("{}.json", storage_type.file_name()));
    let mut storage_file =
        File::create(storage_path.as_path()).map_err(|_| StorageError::FileOpenError {
            message: "Failed to create or open the storage file path",
        })?;

    storage_file
        .write_all(serialized.as_bytes())
        .map_err(|_| StorageError::StorageSaveError {
            message: "Failed to write storage file to path",
        })?;
    Ok(())
}

/// Loads storage data from a [StorageType] inside the working directory
///
/// It might seem weird that a Result with <T, T> is returned here but as the result is only used to determine
/// if default settings were loaded or not this is totally fine and a good way to identify that.
pub fn load_storage_data<T: serde::de::DeserializeOwned + Serialize>(
    storage_type: StorageType,
    default: T,
) -> Result<T, StorageError> {
    let mut storage_path = MOON_WORKING_DIRECTORY.clone();
    storage_path.push(format!("{}.json", storage_type.file_name()));
    let mut storage_file = match File::open(storage_path.as_path()) {
        Ok(file) => file,
        _ => {
            // Save default data if we are unable to open the file
            match save_storage_data(storage_type, default) {
                Ok(_) => {}
                Err(save_err) => return Err(save_err),
            };
            // This error **will** be ignored inside the GUI as it just means that no data has been saved yet
            return Err(StorageError::FileOpenError {
                message: "Failed to open file",
            });
        }
    };

    return match serde_json::from_reader(storage_file) {
        Ok(t) => Ok(t),
        _ => Err(StorageError::JsonSerializeError {
            message: "Failed to deserialize file content of storage type",
        }),
    };
}
