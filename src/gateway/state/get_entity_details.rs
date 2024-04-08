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

    pub fn at_ledger_state(
        mut self,
        at_ledger_state: LedgerStateSelector,
    ) -> Self {
        self.request.at_ledger_state = Some(at_ledger_state);
        self
    }

    #[maybe_async_attr]
    pub async fn execute(
        &self,
    ) -> Result<StateEntityDetails200Response, GatewayApiError> {
        self.client.get_entity_details(self.request.clone()).await
    }
}
