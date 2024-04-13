pub mod builder;
pub mod error;
pub mod mempool;
pub mod models;
pub mod stream;
pub mod transaction;

use self::error::CoreApiError;
use crate::deserialize::from_str;

pub fn match_response<T>(
    text: String,
    status: reqwest::StatusCode,
) -> Result<T, CoreApiError>
where
    T: serde::de::DeserializeOwned,
{
    match status {
        reqwest::StatusCode::OK => {
            Ok(from_str(&text).map_err(|err| CoreApiError::Parsing {
                response: err.path().to_string(),
                serde_error: err.into_inner(),
            })?)
        }
        status if status.is_server_error() => Err(CoreApiError::ServerError(
            from_str(&text).map_err(|err| CoreApiError::Parsing {
                response: err.path().to_string(),
                serde_error: err.into_inner(),
            })?,
        )),
        status if status.is_client_error() => {
            let body = serde_json::from_str(&text).map_err(|err| {
                CoreApiError::Parsing {
                    serde_error: err,
                    response: text.clone(),
                }
            })?;
            Err(CoreApiError::ClientError(body))
        }
        _ => Err(CoreApiError::Unknown),
    }
}
