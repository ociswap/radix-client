use self::gateway::{
    builder::{RequestBuilderAsync, RequestBuilderBlocking},
    error::GatewayApiError,
    match_response,
    models::*,
};
use crate::*;
use chrono::Utc;
use duplicate::duplicate_item;
use maybe_async::*;

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ GatewayClientAsync ]     [ must_be_async ];
    [ GatewayClientBlocking ]  [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn keyvaluestore_keys(
        &self,
        request: GetKeyValueStoreKeysRequestBody,
    ) -> Result<GetKeyValueStoreKeys200ResponseBody, GatewayApiError> {
        let (text, status) =
            self.post("state/key-value-store/keys", request).await?;
        match_response(text, status)
    }
}

// builder

#[duplicate_item(
    request_type client_type ;
    [ RequestBuilderAsync ] [ GatewayClientAsync ] ;
    [ RequestBuilderBlocking ] [ GatewayClientBlocking ] ;
)]
impl client_type {
    pub fn keyvaluestore_keys_builder(
        &self,
        key_value_store_address: &str,
    ) -> request_type<GetKeyValueStoreKeysRequestBody> {
        let request = GetKeyValueStoreKeysRequestBody {
            at_ledger_state: None,
            key_value_store_address: key_value_store_address.to_string(),
            cursor: None,
            limit_per_page: None,
        };
        request_type {
            client: self.clone(),
            request,
        }
    }
}

#[duplicate_item(
    builder_type maybe_async_attr;
    [ RequestBuilderAsync ] [ must_be_async ];
    [ RequestBuilderBlocking ] [ must_be_sync ];
)]
impl builder_type<GetKeyValueStoreKeysRequestBody> {
    pub fn cursor(&mut self, value: String) -> &mut Self {
        self.request.cursor = Some(value);
        self
    }

    pub fn limit_per_page(&mut self, value: u32) -> &mut Self {
        self.request.limit_per_page = Some(value);
        self
    }

    pub fn at_state_version(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            state_version: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn at_timestamp(&mut self, value: chrono::DateTime<Utc>) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            timestamp: Some(value.timestamp() as u64),
            ..Default::default()
        });
        self
    }

    pub fn at_epoch(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            epoch: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn at_round(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            round: Some(value),
            ..Default::default()
        });
        self
    }

    #[maybe_async_attr]
    pub async fn fetch(
        &self,
    ) -> Result<GetKeyValueStoreKeys200ResponseBody, GatewayApiError> {
        self.client.keyvaluestore_keys(self.request.clone()).await
    }
}
