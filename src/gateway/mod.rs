use self::models::GatewayApiError;
pub mod models;
pub mod state;
pub mod status;
pub mod stream;
pub mod transactions;

pub fn match_response<T>(
    text: String,
    status: reqwest::StatusCode,
) -> Result<T, GatewayApiError>
where
    T: serde::de::DeserializeOwned,
{
    match status {
        reqwest::StatusCode::OK => {
            Ok(serde_json::from_str(&text).map_err(|err| {
                GatewayApiError::Parsing {
                    serde_error: err,
                    response: text.clone(),
                }
            })?)
        }
        status if status.is_server_error() => {
            Err(GatewayApiError::ServerError(text.to_string()))
        }
        status if status.is_client_error() => {
            let body = serde_json::from_str(&text).map_err(|err| {
                GatewayApiError::Parsing {
                    serde_error: err,
                    response: text.clone(),
                }
            })?;
            Err(GatewayApiError::ClientError(body))
        }
        _ => Err(GatewayApiError::Unknown),
    }
}
