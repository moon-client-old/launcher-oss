use crate::api::endpoint::{Endpoint, EndpointType};
use crate::api::moon::auth::AuthenticationError::{
    HwidMismatch, InternalServerError, InvalidLoginRequest, InvalidUserAccount, JsonParseFailed,
    NoUserFound, RequestFailed, Unknown,
};
use crate::api::moon::BASE_URL;
use crate::gui::LauncherState;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// Endpoint for authenticating with the servers and receiving a session token
/// There will only be one session token for every user, any old ones are immediately invalidated
/// or 24 hours after creation<br><br>
///
/// # Output Definition
/// This endpoint can produce both a valid JSON outcome and a few error codes<br><br>
///
/// ## Error Code Definition
/// - 0: Invalid login request received, please open a GitHub issue
/// - 1: Please make sure you set both a HWID and a username through the discord bot
/// - 2: Your HWID does not match, please create a HWID reset
/// - 3: No user with the UID you entered could be found, please make sure you entered your UID correctly
/// - 4: Internal server error, please create a ticket
/// <br><br>
///
/// ## Valid Outcome Definition
/// The valid outcome, is as previously mentioned a valid JSON string which looks
/// similar to the following
///
/// ```json
/// {
///     "username": "USERNAME",
///     "rank": "ADMIN",
///     "session_key": "RANDOM_SESSION_KEY",
///     "available_channels": [
///         {
///             "name": "Release",
///             "description": "Example description",
///             "rankRequired": "USER",
///             "latestVersion": "version_01",
///             "lastUpdated": 0,
///             "availableVersions": [
///                 {
///                     "name": "version_01",
///                     "_id": "version01",
///                     "changelog": "No changes :(",
///                     "releasedAt": 0
///                 }
///             ]
///         }
///     ]
/// }
/// ```
/// The **`rankRequired`** field might indicate that ranks are validated on the client-side,
/// this is not the case, the server will only send channels to the user which he has
/// access to, eliminating any malicious actions. Additionally on download the server once again
/// verifies that the user actually has access to the channel which he is trying to download
pub struct AuthenticationEndpointData {
    pub uid: i64,
}

/// Contains the response data as described in the [AuthenticationEndpointData] documentation
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationResponseData {
    pub username: String,
    pub rank: UserRank,
    pub session_key: String,
    pub available_channels: Vec<Channel>,
}

/// The channel struct containing all information about an available channel
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub name: String,
    pub description: String,
    pub rank_required: UserRank,
    pub latest_version: String,
    pub last_updated: i64,
    pub available_versions: Vec<Version>,
}

/// The version struct containing all information about an available channel version
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub changelog: String,
    pub released_at: i64,
}

/// The different ranks which might be required for a channel (unnecessary to include this in the response, might be removed later)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRank {
    User,
    Beta,
    Staff,
    Admin,
}

impl Endpoint for AuthenticationEndpointData {
    fn url(&self) -> String {
        format!("{BASE_URL}auth?uid={}", self.uid)
    }

    /// The authentication endpoint consumes a header named `Launcher-User-Serial` which is
    /// used for serial verification of the account
    ///
    /// While this is rather a basic approach to authentication, nothing more is required here
    /// as you cannot do much in the launcher anyways besides downloading builds and viewing their
    /// changelog
    fn request_type(&self) -> EndpointType {
        EndpointType::Serial
    }

    /// We don't need any additional headers here as the [EndpointType::SERIAL] defines everything
    /// we need for this endpoint anyways
    fn headers(&self) -> Option<HeaderMap> {
        None
    }
}

/// All different errors with their mappings which can occur upon login
#[derive(Debug, Deserialize, Serialize)]
pub enum AuthenticationError {
    RequestFailed,
    JsonParseFailed,
    InvalidLoginRequest { message: &'static str },
    InvalidUserAccount { message: &'static str },
    HwidMismatch { message: &'static str },
    NoUserFound { message: &'static str },
    InternalServerError { message: &'static str },
    Unknown,
}

/// Authenticates with the backend servers
pub async fn authenticate(
    state: &LauncherState,
    uid: i64,
) -> Result<AuthenticationResponseData, AuthenticationError> {
    let endpoint = AuthenticationEndpointData { uid };
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
            "0" => InvalidLoginRequest { message: "Invalid login request received, please open a GitHub issue" },
            "1" => InvalidUserAccount { message: "Please make sure you set both a HWID and a username through the discord bot" },
            "2" => HwidMismatch { message: "Your HWID does not match, please create a HWID reset" },
            "3" => NoUserFound { message: "No user with the UID you entered could be found, please make sure you entered your UID correctly" },
            "4" => InternalServerError { message: "Internal server error, please create a ticket" },
            _ => Unknown
        });
    }

    match serde_json::from_slice::<AuthenticationResponseData>(content.as_bytes()) {
        Ok(parsed) => Ok(parsed),
        _ => Err(JsonParseFailed),
    }
}
