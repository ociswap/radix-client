use self::{
    builder::{AsyncRequestBuilder, BlockingRequestBuilder},
    core::{error::CoreApiError, match_response, models::*},
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
    pub async fn get_committed_transactions(
        &self,
        request: GetCommittedTransactionsRequest,
    ) -> Result<GetCommittedTransactionsRequest, CoreApiError> {
        let (text, status) =
            self.post_request("stream/transactions", request).await?;
        match_response(text, status)
    }
}

// pub struct GetCommittedTransactionsRequest {
//     pub network: String,
//     pub from_state_version: u64,
//     pub limit: u32,
//     pub sbor_format_options: Option<SborFormatOptions>,
//     pub transaction_format_options: Option<TransactionFormatOptions>,
//     pub substate_format_options: Option<SubstateFormatOptions>,
//     pub include_proofs: Option<bool>,
// }

// builder

#[duplicate_item(
    request_type client_type ;
    [ AsyncRequestBuilder ] [ GatewayClientAsync ] ;
    [ BlockingRequestBuilder ] [ GatewayClientBlocking ] ;
)]
impl client_type {
    pub fn get_committed_transactions_builder(
        &self,
        network: String,
        from_state_version: u64,
        limit: u32,
    ) -> request_type<GetCommittedTransactionsRequest> {
        let request = GetCommittedTransactionsRequest {
            network,
            from_state_version,
            limit,
            ..Default::default()
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
impl builder_type<'_, GetCommittedTransactionsRequest> {
    pub fn sbor_format_options(
        &mut self,
        value: SborFormatOptions,
    ) -> &mut Self {
        self.request.sbor_format_options = Some(value);
        self
    }

    pub fn transaction_format_options(
        &mut self,
        value: TransactionFormatOptions,
    ) -> &mut Self {
        self.request.transaction_format_options = Some(value);
        self
    }

    pub fn substate_format_options(
        &mut self,
        value: SubstateFormatOptions,
    ) -> &mut Self {
        self.request.substate_format_options = Some(value);
        self
    }

    pub fn include_proofs(&mut self) -> &mut Self {
        self.request.include_proofs = Some(true);
        self
    }

    #[maybe_async_attr]
    pub async fn execute(
        &self,
    ) -> Result<GetCommittedTransactionsRequest, CoreApiError> {
        self.client
            .get_committed_transactions(self.request.clone())
            .await
    }
}
