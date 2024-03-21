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
    pub async fn transaction_preview_request(
        self,
        network: String,
        manifest: String,
        blobs_hex: Option<Vec<String>>,
        start_epoch_inclusive: i64,
        end_epoch_exclusive: i64,
        notary_public_key: Option<PublicKey>,
        notary_is_signatory: Option<bool>,
        tip_percentage: i32,
        nonce: i64,
        signer_public_keys: Vec<PublicKey>,
        flags: PreviewTransactionFlags,
    ) -> Result<TransactionPreview200ResponseBody, CoreApiError> {
        let body = TransactionPreviewRequestBody {
            network,
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
    pub async fn scrypto_call_preview(
        &self,
        network: String,
        target: TargetIdentifier,
        arguments: Vec<String>,
    ) -> Result<ScryptoCallPreview200ResponseBody, CoreApiError> {
        let body = ScryptoCallPreviewRequestBody {
            network,
            target,
            arguments,
        };
        let (text, status) =
            self.post_request("transaction/call-preview", body).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn transaction_submit_request(
        &self,
        network: String,
        notarized_transaction_hex: String,
    ) -> Result<Transactionsubmit200ResponseBody, CoreApiError> {
        let body = TransactionSubmitRequestBody {
            network,
            notarized_transaction_hex,
        };
        let (text, status) =
            self.post_request("transaction/submit", body).await?;
        match_response(text, status)
    }

    #[maybe_async_attr]
    pub async fn transaction_stream_request(
        &self,
        network: String,
        from_state_version: u64,
        limit: u32,
        sbor_format_options: SborFormatOptions,
        transaction_format_options: TransactionFormatOptions,
        substate_format_options: SubstateFormatOptions,
        include_proofs: bool,
    ) -> Result<TransactionStream200ResponseBody, CoreApiError> {
        let body = TransactionStreamRequestBody {
            network,
            from_state_version,
            limit,
            sbor_format_options,
            transaction_format_options,
            substate_format_options,
            include_proofs,
        };
        let (text, status) =
            self.post_request("stream/transactions", body).await?;
        match_response(text, status)
    }
}
