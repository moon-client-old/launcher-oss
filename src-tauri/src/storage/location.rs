use directories::BaseDirs;
use lazy_static::lazy_static;

use std::fs::create_dir_all;
use std::path::PathBuf;

lazy_static! {
    /// Yes this is fine if it panics the program
    ///
    /// ### Why?
    ///
    /// The launcher won't be able to operate properly if we are unable to create this directory.
    /// It is simply **REQUIRED** to proceed
    pub static ref MOON_WORKING_DIRECTORY: PathBuf = {
        resolve_working_directory().expect("Working directory could not be created")
    };
    pub static ref MINECRAFT_WORKING_DIRECTORY: PathBuf = {
        resolve_minecraft_directory().expect("Minecraft directory could not be created")
    };
}

/// Contains all errors which might happen on creation of the working directory
#[derive(Debug)]
pub enum StorageLocationError {
    BaseDirectoriesMissing,
    UnableToCreateWorkingDirectory,
}

/// Resolves the working directory of the launcher
///
/// The path of this directory differs depending on the operating system the user is on
/// - Windows: %APPDATA%/Roaming/.moon
/// - Linux: /home/%USERNAME%/.moon
pub fn resolve_working_directory() -> Result<PathBuf, StorageLocationError> {
    let user_base_dirs = match BaseDirs::new() {
        Some(dir) => dir,
        _ => return Err(StorageLocationError::BaseDirectoriesMissing),
    };

    let mut working_dir = PathBuf::from(user_base_dirs.data_dir());
    working_dir.push(".moon/");

    create_dir_all(&working_dir)
        .map_err(|_| StorageLocationError::UnableToCreateWorkingDirectory)?;

    Ok(working_dir)
}

/// Resolves the minecraft directory of the launcher
///
/// The path of this directory differs depending on the operating system the user is on
/// - Windows: %APPDATA%/Roaming/.minecraft
/// - Linux: /home/%USERNAME%/.minecraft
pub fn resolve_minecraft_directory() -> Result<PathBuf, StorageLocationError> {
    let user_base_dirs = match BaseDirs::new() {
        Some(dir) => dir,
        _ => return Err(StorageLocationError::BaseDirectoriesMissing),
    };

    let mut minecraft_dir = PathBuf::from(user_base_dirs.data_dir());
    minecraft_dir.push(".minecraft/");

    create_dir_all(&minecraft_dir)
        .map_err(|_| StorageLocationError::UnableToCreateWorkingDirectory)?;

    Ok(minecraft_dir)
}
