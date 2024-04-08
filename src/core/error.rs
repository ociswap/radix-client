use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct ErrorDetails {
    pub error: String,
    pub error_description: String,
}

#[derive(Debug, Deserialize)]
pub enum CoreApiErrorResponse {
    Basic(ErrorData<()>),
    // todo! add more error types
    TransactionSubmit(ErrorData<()>),
    LtsTransactionSubmit(ErrorData<()>),
    StreamTransactions(ErrorData<()>),
    StreamProofs(ErrorData<()>),
}

#[derive(Debug, Deserialize)]
pub struct ErrorData<T> {
    pub code: u16,
    pub message: String,
    pub trace_id: Option<String>,
    pub details: Option<T>,
}

pub enum CoreApiError {
    Network(reqwest::Error),
    Parsing {
        serde_error: serde_json::Error,
        response: String,
    },
    ClientError(CoreApiErrorResponse),
    ServerError(CoreApiErrorResponse),
    Unknown,
}

impl std::fmt::Display for CoreApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoreApiError::Network(e) => write!(f, "Network error: {}", e),
            CoreApiError::Parsing {
                serde_error,
                response,
            } => write!(
                f,
                "Parsing error: {}: Path to wrong field: {:#?}",
                serde_error,
                response.chars().take(1000).collect::<String>().to_string()
            ),
            CoreApiError::ClientError(e) => {
                write!(f, "Client error: {:?}", e)
            }
            CoreApiError::ServerError(e) => {
                write!(f, "Server error: {:?}", e)
            }
            CoreApiError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Debug for CoreApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use the Display implementation
        write!(f, "{}", self)
    }
}

impl Error for CoreApiError {}

impl From<reqwest::Error> for CoreApiError {
    fn from(e: reqwest::Error) -> Self {
        CoreApiError::Network(e)
    }
}
