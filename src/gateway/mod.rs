pub mod builder;
pub mod error;
pub mod models;
pub mod state;
pub mod status;
pub mod stream;
pub mod transaction;

use crate::deserialize::from_str;

use self::error::GatewayApiError;

pub fn match_response<T>(
    text: String,
    status: reqwest::StatusCode,
) -> Result<T, GatewayApiError>
where
    T: serde::de::DeserializeOwned,
{
    match status {
        reqwest::StatusCode::OK => {
            Ok(from_str(&text).map_err(|err| GatewayApiError::Parsing {
                serde_error: err,
                response: text.clone(),
            })?)
        }
        status if status.is_server_error() => {
            Err(GatewayApiError::ServerError(from_str(&text).map_err(
                |err| GatewayApiError::Parsing {
                    serde_error: err,
                    response: text.clone(),
                },
            )?))
        }
        status if status.is_client_error() => {
            Err(GatewayApiError::ClientError(from_str(&text).map_err(
                |err| GatewayApiError::Parsing {
                    serde_error: err,
                    response: text.clone(),
                },
            )?))
        }
        _ => Err(GatewayApiError::Unknown),
    }
}
