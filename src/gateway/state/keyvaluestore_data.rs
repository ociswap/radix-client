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
use serde_json::Value;

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ GatewayClientAsync ]     [ must_be_async ];
    [ GatewayClientBlocking ]  [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn keyvaluestore_data(
        &self,
        request: GetKeyValueStoreDataRequestBody,
    ) -> Result<GetKeyValueStoreData200ResponseBody, GatewayApiError> {
        let (text, status) =
            self.post("state/key-value-store/data", request).await?;
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
    pub fn keyvaluestore_data_builder(
        &self,
        key_value_store_address: String,
    ) -> request_type<GetKeyValueStoreDataRequestBody> {
        let request = GetKeyValueStoreDataRequestBody {
            at_ledger_state: None,
            key_value_store_address,
            keys: vec![],
        };
        request_type {
            client: &self,
            request,
        }
    }
}

#[duplicate_item(
    builder_type maybe_async_attr;
    [ RequestBuilderAsync ] [ must_be_async ];
    [ RequestBuilderBlocking ] [ must_be_sync ];
)]
impl builder_type<'_, GetKeyValueStoreDataRequestBody> {
    pub fn with_keys(
        &mut self,
        value: Vec<StateKeyValueStoreDataRequestKeyItem>,
    ) -> &mut Self {
        self.request.keys = value;
        self
    }

    pub fn add_key_json(&mut self, value: Value) -> &mut Self {
        self.request
            .keys
            .push(StateKeyValueStoreDataRequestKeyItem {
                key_json: Some(value),
                key_hex: None,
            });
        self
    }

    pub fn add_key_hex(&mut self, value: &str) -> &mut Self {
        self.request
            .keys
            .push(StateKeyValueStoreDataRequestKeyItem {
                key_json: None,
                key_hex: Some(value.to_string()),
            });
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
    ) -> Result<GetKeyValueStoreData200ResponseBody, GatewayApiError> {
        self.client.keyvaluestore_data(self.request.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::PUBLIC_GATEWAY_URL, GatewayClientBlocking};

    #[test]
    fn test_keyvaluestore_data() {
        let client = GatewayClientBlocking::new(PUBLIC_GATEWAY_URL.to_string());
        let kvs_address = "internal_keyvaluestore_rdx1kp9qamy3m54cxhple4npsal58x7rur6ev5w2me6ne6zfr47lp6h4cp".to_string();
        let key = client
            .keyvaluestore_keys_builder(&kvs_address)
            .at_state_version(50_000_000)
            .limit_per_page(1)
            .fetch()
            .unwrap();
        let hex_key = key.items.first().unwrap().key.clone().raw_hex;

        let response = client
            .keyvaluestore_data_builder(kvs_address)
            .add_key_hex(&hex_key)
            .fetch();

        println!("{:?}", response);
    }
}
