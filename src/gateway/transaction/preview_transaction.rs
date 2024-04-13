use self::gateway::{
    builder::{RequestBuilderAsync, RequestBuilderBlocking},
    error::GatewayApiError,
    match_response,
    models::*,
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
    pub async fn preview_transaction(
        &self,
        request: TransactionPreviewRequestBody,
    ) -> Result<TransactionPreview200ResponseBody, GatewayApiError> {
        let (text, status) = self.post("transaction/preview", request).await?;
        match_response(text, status)
    }
}

// builder

#[duplicate_item(
    request_type client_type ;
    [ RequestBuilderAsync ] [ GatewayClientAsync ] ;
    [ RequestBuilderBlocking ] [ GatewayClientBlocking ] ;
)]
impl client_type {
    pub fn preview_transaction_builder(
        &self,
        manifest: String,
        start_epoch_inclusive: i64,
        end_epoch_exclusive: i64,
        nonce: String,
        signer_public_keys: Vec<PublicKey>,
    ) -> request_type<TransactionPreviewRequestBody> {
        let request = TransactionPreviewRequestBody {
            manifest,
            start_epoch_inclusive,
            end_epoch_exclusive,
            nonce,
            signer_public_keys,
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
    pub fn blobs_hex(mut self, value: Vec<String>) -> Self {
        self.request.blobs_hex = Some(value);
        self
    }

    pub fn notary_public_key(mut self, value: PublicKey) -> Self {
        self.request.notary_public_key = Some(value);
        self
    }

    pub fn notary_is_signatory(mut self, value: bool) -> Self {
        self.request.notary_is_signatory = Some(value);
        self
    }

    pub fn tip_percentage(mut self, value: i32) -> Self {
        self.request.tip_percentage = value;
        self
    }

    pub fn use_free_credit(mut self) -> Self {
        self.request.flags.use_free_credit = true;
        self
    }

    pub fn assume_all_signature_proofs(mut self) -> Self {
        self.request.flags.assume_all_signature_proofs = true;
        self
    }

    pub fn skip_epoch_check(mut self) -> Self {
        self.request.flags.skip_epoch_check = true;
        self
    }

    #[maybe_async_attr]
    pub async fn fetch(
        self,
    ) -> Result<TransactionPreview200ResponseBody, GatewayApiError> {
        self.client.preview_transaction(self.request.clone()).await
    }
}
