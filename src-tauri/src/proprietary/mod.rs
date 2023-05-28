use crate::storage::location::MOON_WORKING_DIRECTORY;
use downloader::Downloader;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

pub const PROPRIETARY_LIBRARY_VERSION: &str = "launcher_lib-1.0";
pub const BASE_DOWNLOAD_URL: &str = "https://cdn.moonclient.xyz/launcher/proprietary/";

/// Internal library enum to provide proper error messages for serial fetching
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
        let mut downloader = Downloader::builder()
            .download_folder(MOON_WORKING_DIRECTORY.as_path())
            .parallel_requests(1)
            .build()
            .unwrap();
        let download_link = downloader::download::Download::new(
            format!("{BASE_DOWNLOAD_URL}{library_name}").as_str(),
        );
        downloader
            .download(&[download_link])
            .expect("Failed to download library file");
    }

    // Hack the mainframe by loading the library from a specific path (not really a hack, but still)
    unsafe {
        match Library::new(library_path) {
            Ok(lib) => return lib,
            _ => panic!("Unable to load proprietary library, please open an issue on GitHub"),
        }
    }
}
