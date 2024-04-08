use self::{
    builder::{AsyncRequestBuilder, BlockingRequestBuilder},
    gateway::{error::GatewayApiError, match_response, models::*},
};
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
    pub async fn get_kvs_data(
        &self,
        at_ledger_state: Option<LedgerStateSelector>,
        key_value_store_address: String,
        keys: Vec<StateKeyValueStoreDataRequestKeyItem>,
    ) -> Result<GetKeyValueStoreData200ResponseBody, GatewayApiError> {
        let request = GetKeyValueStoreDataRequestBody {
            at_ledger_state,
            key_value_store_address,
            keys,
        };
        let (text, status) = self
            .post_request("state/key-value-store/data", request)
            .await?;
        match_response(text, status)
    }
}

// builder
