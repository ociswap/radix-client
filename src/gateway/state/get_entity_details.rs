use self::{
    builder::{AsyncRequestBuilder, BlockingRequestBuilder},
    gateway::{error::GatewayApiError, match_response, models::*},
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
    pub async fn get_entity_details(
        &self,
        request: StateEntityDetailsRequest,
    ) -> Result<StateEntityDetails200Response, GatewayApiError> {
        let (text, status) =
            self.post_request("state/entity/details", request).await?;
        match_response(text, status)
    }
}

// builder

#[duplicate_item(
    request_type client_type ;
    [ AsyncRequestBuilder ] [ GatewayClientAsync ] ;
    [ BlockingRequestBuilder ] [ GatewayClientBlocking ] ;
)]
impl client_type {
    pub fn get_entity_details_builder(
        &self,
        addresses: Vec<String>,
    ) -> request_type<StateEntityDetailsRequest> {
        let request = StateEntityDetailsRequest {
            addresses,
            aggregation_level: None,
            opt_ins: None,
            at_ledger_state: None,
        };
        request_type {
            client: &self,
            request,
        }
    }
}

#[duplicate_item(
    builder_type maybe_async_attr;
    [ AsyncRequestBuilder ] [ must_be_async ];
    [ BlockingRequestBuilder ] [ must_be_sync ];
)]
impl builder_type<'_, StateEntityDetailsRequest> {
    pub fn aggregation_level(
        mut self,
        aggregation_level: AggregationLevel,
    ) -> Self {
        self.request.aggregation_level = Some(aggregation_level);
        self
    }

    pub fn opt_ins(mut self, opt_ins: StateEntityDetailsRequestOptIns) -> Self {
        self.request.opt_ins = Some(opt_ins);
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
    ) -> Result<StateEntityDetails200Response, GatewayApiError> {
        self.client.get_entity_details(self.request.clone()).await
    }
}
