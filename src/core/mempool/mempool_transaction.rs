use self::core::{error::CoreApiError, match_response, models::*};
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
    pub async fn mempool_transaction(
        &self,
        network: String,
        payload_hashes: Vec<String>,
    ) -> Result<GetMempoolTransaction200Response, CoreApiError> {
        let request = GetMempoolTransactionRequest {
            network,
            payload_hashes,
        };
        let (text, status) = self.post("mempool/transaction", request).await?;
        match_response(text, status)
    }
}
