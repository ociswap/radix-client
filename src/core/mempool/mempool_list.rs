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
    pub async fn mempool_list(
        &self,
        network: String,
    ) -> Result<GetMempoolList200Response, CoreApiError> {
        let request = GetMempoolListRequest { network };
        let (text, status) = self.post("mempool/list", request).await?;
        match_response(text, status)
    }
}
