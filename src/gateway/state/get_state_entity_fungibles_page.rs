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
