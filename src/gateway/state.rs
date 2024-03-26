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
    pub async fn get_entity_details(
        &self,
        at_ledger_state: Option<LedgerStateSelector>,
        opt_ins: Option<StateEntityDetailsRequestOptIns>,
        addresses: Vec<String>,
        aggregation_level: Option<AggregationLevel>,
    ) -> Result<StateEntityDetails200Response, GatewayApiError> {
        let request = StateEntityDetailsRequest {
            at_ledger_state,
            opt_ins,
            addresses,
            aggregation_level,
        };
        let (text, status) =
            self.post_request("state/entity/details", request).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn get_state_entity_fungibles_page(
        &self,
        at_ledger_state: Option<LedgerStateSelector>,
        cursor: Option<String>,
        limit_per_page: Option<u32>,
        address: String,
        aggregation_level: Option<AggregationLevel>,
        opt_ins: Option<StateEntityFungiblesPageRequestOptIns>,
    ) -> Result<StateEntityFungiblesPage200Response, GatewayApiError> {
        let request = StateEntityFungiblesPageRequest {
            at_ledger_state,
            cursor,
            limit_per_page,
            address,
            aggregation_level,
            opt_ins,
        };
        let (text, status) = self
            .post_request("state/entity/page/fungibles", request)
            .await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn get_kvs_keys(
        &self,
        at_ledger_state: Option<LedgerStateSelector>,
        cursor: Option<String>,
        limit_per_page: Option<u32>,
        key_value_store_address: String,
    ) -> Result<GetKeyValueStoreKeys200ResponseBody, GatewayApiError> {
        let request = GetKeyValueStoreKeysRequestBody {
            at_ledger_state,
            cursor,
            limit_per_page,
            key_value_store_address,
        };
        let (text, status) = self
            .post_request("state/key-value-store/keys", request)
            .await?;
        match_response(text, status)
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_state_entity_fungibles_page() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let response = client.get_state_entity_fungibles_page(
            None,
            None,
            None,
            "pool_rdx1c5shqw3yq5s7l6tr2k9h9k68cqsju9upr2wq5672rw5gt6y5s6rypj"
                .to_string(),
            None,
            None,
        ).unwrap();
        assert_eq!(response.ledger_state.network, "mainnet".to_string());
        assert!(response.total_count.unwrap() > 0);
    }

    #[test]
    fn test_get_state_entity_details() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let response = client.get_entity_details(
            None,
            None,
            vec![
                "component_rdx1czmr0nmtky4qfe7sn7dke6nxl4fs3pnp3wmre92cvaz06eyvwmv42y"
                    .to_string(),
            ],
            None,
        ).unwrap();
        println!("{:?}", response);
    }

    #[test]
    fn test_get_kvs_keys_and_data() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let response = client.get_kvs_keys(
            None,
            None,
            None,
            "internal_keyvaluestore_rdx1kp9qamy3m54cxhple4npsal58x7rur6ev5w2me6ne6zfr47lp6h4cp".to_string(),
        ).unwrap();
        println!("{:#?}", response);
        let keys: Vec<_> = response
            .items
            .iter()
            .map(|item| {
                let key_hex = item.key.as_object().unwrap()["raw_hex"]
                    .to_string()
                    .trim_matches('"')
                    .to_string();
                StateKeyValueStoreDataRequestKeyItem {
                    key_hex: Some(key_hex),
                    key_json: None,
                }
            })
            .collect();
        let response = client.get_kvs_data(
            None,
            "internal_keyvaluestore_rdx1kp9qamy3m54cxhple4npsal58x7rur6ev5w2me6ne6zfr47lp6h4cp".to_string(),
            keys,
        ).unwrap();
        let values: Vec<_> = response
            .entries
            .iter()
            .map(|entry| {
                (
                    entry.key.as_object().unwrap()["programmatic_json"]
                        .as_object()
                        .unwrap()["value"]
                        .to_string()
                        .trim_matches('\"')
                        .to_string(),
                    entry.value.as_object().unwrap()["programmatic_json"]
                        .as_object()
                        .unwrap()["fields"]
                        .as_array()
                        .unwrap()[0]
                        .as_object()
                        .unwrap()["value"]
                        .to_string()
                        .trim_matches('\"')
                        .to_string(),
                )
            })
            .collect();
        println!("{:#?}", values);
    }
}
