use self::core::{error::CoreApiError, match_response, models::*};
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
    pub async fn transaction_submit(
        &self,
        network: String,
        notarized_transaction_hex: String,
    ) -> Result<Transactionsubmit200ResponseBody, CoreApiError> {
        let body = TransactionSubmitRequestBody {
            network,
            notarized_transaction_hex,
        };
        let (text, status) = self.post("transaction/submit", body).await?;
        match_response(text, status)
    }
}
