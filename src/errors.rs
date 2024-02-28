use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Zebedee Error
#[derive(thiserror::Error, Debug)]
pub enum ZebedeeError {
    /// Error from reqwest crate which is used to make HTTP requests
    #[error("{0}")]
    InvalidRequest(#[from] reqwest::Error),
    /// Serde json Errors when parsing
    #[error("Unable to parse json: {0}")]
    InvalidJson(#[from] serde_json::Error),
    /// Serde json Errors when parsing
    #[error("{0}")]
    Validate(#[from] validator::ValidationErrors),
    /// Error messages from Zebedee REST API
    #[error("{0}")]
    Api(ApiError),
    /// Internal Error messages
    #[error("{0}")]
    Msg(ErrorMsg),
}

/// Zebedee Rest API error message
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ApiError {
    /// Error message
    pub message: Option<String>,
    /// Status of API call
    pub success: bool,
}

/// General Error messages
#[derive(thiserror::Error, Debug)]
pub enum ErrorMsg {
    /// Bad Gamertag Payment format
    #[error("Bad Gamertag Payment format {0}")]
    BadGamerTagFormat(String),
    /// Bad payload data
    #[error("Bad payload data {0}")]
    BadPayloadData(String),
    /// Bad LN Address
    #[error("Bad LN Address {0}, ValidationError {1}")]
    BadLnAddress(String, String),
}

impl From<ErrorMsg> for ZebedeeError {
    fn from(value: ErrorMsg) -> Self {
        ZebedeeError::Msg(value)
    }
}

impl From<ApiError> for ZebedeeError {
    fn from(value: ApiError) -> Self {
        ZebedeeError::Api(value)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.message {
            Some(s) => f.write_str(s.as_str()),
            None => f.write_str("No Message Returned"),
        }
    }
}
