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
    pub async fn get_state_entity_fungibles_page(
        &self,
        request: StateEntityFungiblesPageRequest,
    ) -> Result<StateEntityFungiblesPage200Response, GatewayApiError> {
        let (text, status) = self
            .post_request("state/entity/page/fungibles", request)
            .await?;
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
    pub fn get_state_entity_fungibles_page_builder(
        &self,
        entity_address: &str,
    ) -> request_type<StateEntityFungiblesPageRequest> {
        let request = StateEntityFungiblesPageRequest {
            address: entity_address.to_string(),
            at_ledger_state: None,
            cursor: None,
            limit_per_page: None,
            aggregation_level: None,
            opt_ins: None,
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
impl builder_type<'_, StateEntityFungiblesPageRequest> {
    pub fn aggregation_level(
        &mut self,
        aggregation_level: AggregationLevel,
    ) -> &mut Self {
        self.request.aggregation_level = Some(aggregation_level);
        self
    }

    pub fn with_explicit_metadata(
        &mut self,
        properties: Vec<String>,
    ) -> &mut Self {
        self.request.opt_ins = Some(StateEntityFungiblesPageRequestOptIns {
            explicit_metadata: properties,
        });
        self
    }

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
    pub async fn execute(
        &self,
    ) -> Result<StateEntityFungiblesPage200Response, GatewayApiError> {
        self.client
            .get_state_entity_fungibles_page(self.request.clone())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::PUBLIC_GATEWAY_URL, GatewayClientBlocking};

    #[test]
    fn test_get_state_entity_fungibles_page_builder() {
        let client = GatewayClientBlocking::new(PUBLIC_GATEWAY_URL.to_string());
        let response = client
            .get_state_entity_fungibles_page_builder
            ("component_rdx1cz89w3ecvh9jvdd892vycs44rr042lteg75zgdydq9csn5d87snvdw")
            .at_state_version(50_000_000)
            .limit_per_page(1)
            .execute();

        println!("{:?}", response);
        assert!(response.is_ok());
    }
}
