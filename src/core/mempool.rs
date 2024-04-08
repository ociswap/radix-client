use super::error::CoreApiError;
use super::match_response;
use super::models::*;
use crate::CoreClientAsync;
use crate::CoreClientBlocking;
use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ CoreClientAsync ]         [ must_be_async ];
    [ CoreClientBlocking ]      [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn get_mempool_list(
        &self,
        request: GetMempoolListRequest,
    ) -> Result<GetMempoolList200Response, CoreApiError> {
        let (text, status) = self.post_request("mempool/list", request).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn get_mempool_transactions(
        &self,
        request: GetMempoolTransactionsRequest,
    ) -> Result<GetMempoolTransactions200Response, CoreApiError> {
        let (text, status) =
            self.post_request("mempool/transaction", request).await?;
        match_response(text, status)
    }
}

#[cfg(test)]
mod tests {
    use super::GetMempoolListRequest;
    use crate::{constants::PUBLIC_CORE_URL, *};

    #[test]
    fn get_mempool_list() {
        let client = CoreClientBlocking::new(PUBLIC_CORE_URL.to_string());
        let request = GetMempoolListRequest {
            network: "mainnet".to_string(),
        };
        let resp = client
            .get_mempool_list(request)
            .expect("Failed to get mempool list");
        println!("{:?}", resp);
    }
}
