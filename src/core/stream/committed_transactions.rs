use self::core::{
    builder::{RequestBuilderAsync, RequestBuilderBlocking},
    error::CoreApiError,
    match_response,
    models::*,
};
use crate::*;
use duplicate::duplicate_item;
use maybe_async::*;

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ CoreClientAsync ]     [ must_be_async ];
    [ CoreClientBlocking ]  [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn committed_transactions(
        &self,
        request: GetCommittedTransactionsRequest,
    ) -> Result<GetCommittedTransactions200ResponseBody, CoreApiError> {
        let (text, status) = self.post("stream/transactions", request).await?;
        match_response(text, status)
    }
}

#[duplicate_item(
    request_type client_type ;
    [ RequestBuilderAsync ] [ CoreClientAsync ] ;
    [ RequestBuilderBlocking ] [ CoreClientBlocking ] ;
)]
impl client_type {
    pub fn committed_transactions_builder(
        &self,
    ) -> request_type<GetCommittedTransactionsRequest> {
        let request = GetCommittedTransactionsRequest {
            ..Default::default()
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
impl builder_type<GetCommittedTransactionsRequest> {
    pub fn network(&mut self, value: String) -> &mut Self {
        self.request.network = value;
        self
    }

    pub fn limit(&mut self, value: u32) -> &mut Self {
        self.request.limit = value;
        self
    }

    pub fn from_state_version(&mut self, value: u64) -> &mut Self {
        self.request.from_state_version = value;
        self
    }

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
    pub async fn fetch(
        &self,
    ) -> Result<GetCommittedTransactions200ResponseBody, CoreApiError> {
        self.client
            .committed_transactions(self.request.clone())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::core::models::TransactionFormatOptions;
    use crate::*;
    use constants::PUBLIC_CORE_URL;

    #[test]
    fn simple() {
        let client = CoreClientBlocking::new(PUBLIC_CORE_URL.to_string());
        let response = client
            .committed_transactions_builder()
            .from_state_version(1000000)
            .transaction_format_options(TransactionFormatOptions {
                balance_changes: true,
                ..Default::default()
            })
            .fetch()
            .unwrap();
        println!("{:#?}", response);
    }
}
