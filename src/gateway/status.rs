use super::match_response;
use super::models::*;
use crate::GatewayClientAsync;
use crate::GatewayClientBlocking;
use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gateway_status() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let response = client.get_gateway_status().unwrap();
        println!("{:?}", response);
    }
}
