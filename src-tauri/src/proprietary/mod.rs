use crate::storage::location::MOON_WORKING_DIRECTORY;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use reqwest::{Client, StatusCode};
use tauri::async_runtime::block_on;
use tokio::task::block_in_place;

pub const PROPRIETARY_LIBRARY_VERSION: &str = "launcher_lib-1.0";
pub const BASE_DOWNLOAD_URL: &str = "https://cdn.moonclient.xyz/launcher/proprietary/";

/// Internal library enum to provide proper error messages for serial fetching
#[allow(dead_code)] // This is constructed by the proprietary library
#[derive(Debug)]
pub enum SerialProvideError {
    OsError(String),
}

lazy_static! {
    pub static ref PROPRIETARY_LIBRARY: Library = load_proprietary_library();
}

/// Fetches the serial of the current device
///
/// # Safety
/// This function makes an unsafe call to an external library
pub unsafe fn fetch_serial() -> Result<String, SerialProvideError> {
    let function_result: Symbol<unsafe extern "C" fn() -> Result<String, SerialProvideError>> =
        match PROPRIETARY_LIBRARY.get(b"fetch_serial") {
            Ok(func) => func,
            Err(e) => {
                // We cannot continue if this function cannot be loaded, it is necessary for the authentication process
                panic!("Unable to find fetch_seral in proprietary library {}", e)
            }
        };

    function_result()
}

/// Loads the proprietary library of the launcher
fn load_proprietary_library() -> Library {
    let mut library_path = MOON_WORKING_DIRECTORY.clone();

    // Choose the library name depending on the os we are running on
    let library_name = if cfg!(windows) {
        format!("{}.dll", PROPRIETARY_LIBRARY_VERSION)
    } else {
        format!("lib{}.so", PROPRIETARY_LIBRARY_VERSION)
    };
    library_path.push(&library_name);

    // We have to download the library if it does not exist
    if !library_path.exists() {
        match block_in_place(|| {
            block_on(
                Client::new()
                    .get(format!("{BASE_DOWNLOAD_URL}{library_name}"))
                    .send(),
            )
        }) {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    panic!("Proprietary library returned unexpected status code: \"{}\", please open an issue on GitHub", res.status());
                }

                let data = block_in_place(|| block_on(res.bytes()))
                    .expect("Failed to read proprietary library from response, please open an issue on GitHub");

                if let Err(e) = std::fs::write(&library_path, data) {
                    panic!("Failed to write proprietary library: \"{}\", please open an issue on GitHub", e);
                }
            }

            Err(e) => {
                panic!("Failed to download proprietary library: \"{}\", please open an issue on GitHub", e);
            }
        };
    }

    // Hack the mainframe by loading the library from a specific path (not really a hack, but still)
    unsafe {
        match Library::new(library_path) {
            Ok(lib) => lib,
            Err(e) => {
                panic!(
                    "Unable to load proprietary library: \"{}\", please open an issue on GitHub",
                    e
                );
            }
        }
    }
}
