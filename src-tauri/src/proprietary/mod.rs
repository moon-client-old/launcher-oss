/// Internal library enum to provide proper error messages for serial fetching
#[derive(Debug)]
pub enum SerialProvideError {
    OsError(String),
}

/// Contains all "proprietary" code for the launcher in form of a dll
#[allow(dead_code)]
#[link(name = "launcher_lib", kind = "dylib")]
extern "system" {
    /// Fetches the device serial which is required for authenticating with the servers
    #[link_name = "fetch_serial"]
    pub fn fetch_serial() -> Result<String, SerialProvideError>;
}