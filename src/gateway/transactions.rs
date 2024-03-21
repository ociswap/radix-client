use super::models::*;
use crate::gateway::match_response;
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
    pub async fn transaction_preview_request(
        &self,
        manifest: String,
        blobs_hex: Option<Vec<String>>,
        start_epoch_inclusive: i64,
        end_epoch_exclusive: i64,
        notary_public_key: Option<PublicKey>,
        notary_is_signatory: Option<bool>,
        tip_percentage: i32,
        nonce: String,
        signer_public_keys: Vec<PublicKey>,
        flags: PreviewTransactionFlags,
    ) -> Result<TransactionPreview200ResponseBody, GatewayApiError> {
        let body = TransactionPreviewRequestBody {
            manifest,
            blobs_hex,
            start_epoch_inclusive,
            end_epoch_exclusive,
            notary_is_signatory,
            notary_public_key,
            tip_percentage,
            nonce,
            signer_public_keys,
            flags,
        };
        let (text, status) =
            self.post_request("transaction/preview", body).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn transaction_submit_request(
        &self,
        notarized_transaction_hex: String,
    ) -> Result<Transactionsubmit200ResponseBody, GatewayApiError> {
        let body = TransactionSubmitRequestBody {
            notarized_transaction_hex,
        };
        let (text, status) =
            self.post_request("transaction/submit", body).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn transaction_stream_request(
        &self,
        at_ledger_state: Option<LedgerStateSelector>,
        from_ledger_state: Option<LedgerStateSelector>,
        cursor: Option<String>,
        limit_per_page: Option<u32>,
        kind_filter: TransactionKindFilter,
        manifest_accounts_withdrawn_from_filter: Option<Vec<String>>,
        manifest_accounts_deposited_into_filter: Option<Vec<String>>,
        manifest_resources_filter: Option<Vec<String>>,
        affected_global_entities_filter: Option<Vec<String>>,
        order: Option<Order>,
        opt_ins: Option<TransactionStreamOptIns>,
    ) -> Result<TransactionStream200ResponseBody, GatewayApiError> {
        let body = TransactionStreamRequestBody {
            at_ledger_state,
            from_ledger_state,
            cursor,
            limit_per_page,
            kind_filter,
            manifest_accounts_withdrawn_from_filter,
            manifest_accounts_deposited_into_filter,
            manifest_resources_filter,
            affected_global_entities_filter,
            order,
            opt_ins,
        };
        let (text, status) =
            self.post_request("stream/transactions", body).await?;
        match_response(text, status)
    }
}
