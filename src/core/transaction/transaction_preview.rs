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
    pub async fn transaction_preview(
        &self,
        request: TransactionPreviewRequestBody,
    ) -> Result<TransactionPreview200ResponseBody, CoreApiError> {
        let (text, status) =
            self.post_request("transaction/preview", request).await?;
        match_response(text, status)
    }
}

// builder

#[duplicate_item(
    request_type client_type ;
    [ RequestBuilderAsync ] [ CoreClientAsync ] ;
    [ RequestBuilderBlocking ] [ CoreClientBlocking ] ;
)]
impl client_type {
    pub fn transaction_preview_builder(
        &self,
        manifest: String,
        start_epoch_inclusive: i64,
        end_epoch_exclusive: i64,
        nonce: i64,
        signer_public_keys: Vec<PublicKey>,
        network: String,
        tip_percentage: i32,
    ) -> request_type<TransactionPreviewRequestBody> {
        let request = TransactionPreviewRequestBody {
            end_epoch_exclusive,
            nonce,
            start_epoch_inclusive,
            tip_percentage,
            manifest,
            signer_public_keys,
            network,
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
    [ RequestBuilderAsync ] [ must_be_async ];
    [ RequestBuilderBlocking ] [ must_be_sync ];
)]
impl builder_type<'_, TransactionPreviewRequestBody> {
    pub fn blobs_hex(&mut self, value: Vec<String>) -> &mut Self {
        self.request.blobs_hex = Some(value);
        self
    }

    pub fn notary_public_key(&mut self, value: PublicKey) -> &mut Self {
        self.request.notary_public_key = Some(value);
        self
    }

    pub fn notary_is_signatory(&mut self, value: bool) -> &mut Self {
        self.request.notary_is_signatory = Some(value);
        self
    }

    // pub struct PreviewTransactionFlags {
    //     pub use_free_credit: bool,
    //     pub assume_all_signature_proofs: bool,
    //     pub skip_epoch_check: bool,
    // }

    pub fn use_free_credit(&mut self) -> &mut Self {
        self.request.flags.use_free_credit = true;
        self
    }

    pub fn assume_all_signature_proofs(&mut self) -> &mut Self {
        self.request.flags.assume_all_signature_proofs = true;
        self
    }

    pub fn skip_epoch_check(&mut self) -> &mut Self {
        self.request.flags.skip_epoch_check = true;
        self
    }

    #[maybe_async_attr]
    pub async fn execute(
        &self,
    ) -> Result<TransactionPreview200ResponseBody, CoreApiError> {
        self.client.transaction_preview(self.request.clone()).await
    }
}
