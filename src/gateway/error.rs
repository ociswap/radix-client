use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ErrorDetails {
    EntityNotFoundError,
    InvalidEntityError,
    NotSyncedUpError,
    InvalidRequestError,
    InvalidTransactionError,
    TransactionNotFoundError,
    InternalServerError,
}

#[derive(Debug, Deserialize)]
pub struct GatewayApiErrorResponse {
    pub message: String,
    pub code: Option<u16>,
    pub details: Option<ErrorDetails>,
    pub trace_id: Option<String>,
}

pub enum GatewayApiError {
    Network(reqwest::Error),
    Parsing {
        serde_error: serde_path_to_error::Error<serde_json::Error>,
        response: String,
    },
    ClientError(GatewayApiErrorResponse),
    ServerError(GatewayApiErrorResponse),
    Unknown,
}

impl std::fmt::Display for GatewayApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GatewayApiError::Network(e) => write!(f, "Network error: {}", e),
            GatewayApiError::Parsing {
                serde_error,
                response,
            } => write!(
                f,
                "Parsing error: {}:\nResponse: {}",
                serde_error, response
            ),
            GatewayApiError::ClientError(e) => {
                write!(f, "Client error: {:?}", e)
            }
            GatewayApiError::ServerError(e) => {
                write!(f, "Server error: {:?}", e)
            }
            GatewayApiError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Debug for GatewayApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the Display implementation
        write!(f, "{}", self)
    }
}

impl Error for GatewayApiError {}

impl From<reqwest::Error> for GatewayApiError {
    fn from(e: reqwest::Error) -> Self {
        GatewayApiError::Network(e)
    }
}
