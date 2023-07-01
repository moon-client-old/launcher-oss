use crate::api::endpoint::Endpoint;
use crate::api::endpoint::EndpointType::Serial;
use crate::gui::LauncherState;
use lazy_static::lazy_static;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use tauri::http::header::HeaderValue;

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0";
const FALLBACK_SERIAL: &str = "INVALID";

lazy_static! {
    static ref REQWEST_CLIENT: Client = Client::new();
}

/// Creates a request using a given [Endpoint], a launcher state must be provided as it contains crucial
/// information such as the user serial
///
/// TODO: Add support for more than just GET (we currently don't need more)
pub async fn create_request(
    state: &LauncherState,
    endpoint: impl Endpoint,
) -> Result<Response, reqwest::Error> {
    let hwid = state.serial.as_str();

    let mut request_headers = endpoint.headers().unwrap_or(HeaderMap::new());
    request_headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

    // If the request wants a serial code header we have to put it now
    if endpoint.request_type() == Serial {
        request_headers.insert(
            "Launcher-User-Serial",
            HeaderValue::from_str(hwid).unwrap_or(HeaderValue::from_static(FALLBACK_SERIAL)),
        );
    }

    REQWEST_CLIENT
        .get(endpoint.url())
        .headers(request_headers)
        .send()
        .await
}
