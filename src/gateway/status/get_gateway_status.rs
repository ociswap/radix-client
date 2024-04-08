use self::gateway::{error::GatewayApiError, match_response, models::*};
use crate::*;
use duplicate::duplicate_item;
use maybe_async::*;

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ GatewayClientAsync ]     [ must_be_async ];
    [ GatewayClientBlocking ]  [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn get_gateway_status(
        &self,
    ) -> Result<GetGatewayStatus200Response, GatewayApiError> {
        let (text, status) = self
            .post_request("status/gateway-status", serde_json::Value::Null)
            .await?;
        match_response(text, status)
    }
}

// builder
