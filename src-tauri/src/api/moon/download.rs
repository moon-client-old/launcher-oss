use crate::api::endpoint::{Endpoint, EndpointType};
use crate::api::moon::download::DownloadRequestError::{
    InsufficientPermissions, InternalServerError, InvalidSession, InvalidUserAccount,
    JsonParseError, RateLimited, RequestFailed, UnknownError,
};
use crate::api::moon::BASE_URL;
use crate::gui::LauncherState;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::Deserialize;

/// Endpoint for requesting a specific version from the backend server
/// Every user can only request a download every 60 seconds to reduce traffic and general
/// cpu usage on our servers<br><br>
///
/// # Output Definition
/// This endpoint can both produce a valid JSON outcome and a few error codes<br><br>
///
/// ## Error Code Definition
/// - 0: Your session expired, please restart the launcher
/// - 1: You are currently rate-limited, please wait one minute
/// - 2: Internal server error, please create a ticket
/// - 3: Please make sure you set both a HWID and a username through the discord bot
/// - 4: You don't have enough permissions to download this channel
/// <br><br>
///
/// ## Valid Outcome Definition
/// The valid outcome, is as previously mentioned a valid JSON string which looks
/// similar to the following
///
/// ```json
/// {
///     "download_link": "https://backend.moonclient.xyz/api/v1/launcher/download/get/DOWNLOAD_CODE"
/// }
/// ```
pub struct DownloadRequestEndpointData {
    pub session_token: String,
    pub channel_name: String,
    pub channel_version: String,
}

/// Contains the response data as described in the [DownloadRequestEndpointData] documentation
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DownloadResponseData {
    download_link: String,
}

impl Endpoint for DownloadRequestEndpointData {
    fn url(&self) -> String {
        format!(
            "{BASE_URL}download/request?session_token={}&channel_name={}&channel_version={}",
            self.session_token, self.channel_name, self.channel_version
        )
    }

    /// We don't need any authentication headers
    fn request_type(&self) -> EndpointType {
        EndpointType::Normal
    }

    /// We don't need any special headers for this as the session token is passed
    /// through the query
    fn headers(&self) -> Option<HeaderMap> {
        None
    }
}

/// All different errors with their mappings which can occur upon download requesting
#[allow(dead_code)]
#[derive(Debug)]
pub enum DownloadRequestError {
    RequestFailed,
    JsonParseError,
    InvalidSession { message: &'static str },
    RateLimited { message: &'static str },
    InternalServerError { message: &'static str },
    InvalidUserAccount { message: &'static str },
    InsufficientPermissions { message: &'static str },
    UnknownError,
}

/// Authenticates with the backend servers
#[allow(dead_code)]
pub async fn request_download(
    state: &LauncherState,
    channel_name: String,
    channel_version: String,
) -> Result<DownloadResponseData, DownloadRequestError> {
    let endpoint = DownloadRequestEndpointData {
        session_token: state.session_token.clone(),
        channel_name,
        channel_version,
    };

    let response = crate::api::requester::create_request(state, endpoint)
        .await
        .map_err(|_| RequestFailed)?;

    let status = response.status();
    let content = response
        .text_with_charset("UTF-8")
        .await
        .map_err(|_| RequestFailed)?;

    // Handle error mappings if status code is not ok
    if status != StatusCode::OK {
        return Err(match content.as_str() {
            "0" => InvalidSession {
                message: "Your session expired, please restart the launcher",
            },
            "1" => RateLimited {
                message: "You are currently rate-limited, please wait one minute",
            },
            "2" => InternalServerError {
                message: "Internal server error, please create a ticket",
            },
            "3" => InvalidUserAccount {
                message:
                    "Please make sure you set both a HWID and a username through the discord bot",
            },
            "4" => InsufficientPermissions {
                message: "You don't have enough permissions to download this ",
            },
            _ => UnknownError,
        });
    }

    match serde_json::from_slice::<DownloadResponseData>(content.as_bytes()) {
        Ok(parsed) => Ok(parsed),
        _ => Err(JsonParseError),
    }
}
