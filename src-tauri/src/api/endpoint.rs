use reqwest::header::HeaderMap;

/// All different endpoint types available
///
/// ### Explanation
/// - NORMAL: A default http request with no extra properties such as additional headers
/// - SERIAL: A http request with the user serial attached as a header
#[derive(PartialEq)]
pub enum EndpointType {
    NORMAL,
    SERIAL,
}

/// The base endpoint trait
///
/// Every endpoint has to individually parse the response received from the server, while it might
/// seem like a good idea to abstract this its simply impossible. The various types of requests and responses
/// we have make this impossible, especially as other websites won't have the same error mapping system the actual
/// backend server does. Writing an abstraction for this is simply overkill and unnecessary.
pub trait Endpoint {
    /// Returns the endpoint to be requested
    /// The base api url is **not** automatically prepended, you have to add it manually
    fn url(&self) -> String;

    /// Returns the request type of the endpoint
    fn request_type(&self) -> EndpointType;

    /// Returns optional headers which might need to be added, is allowed to be None
    fn headers(&self) -> Option<HeaderMap>;
}
