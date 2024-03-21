use super::models::*;
use crate::CoreClientAsync;
use crate::CoreClientBlocking;
use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};

#[duplicate_item(
    stream_type                     client_type ;
    [ TransactionStreamAsync ]      [ CoreClientAsync ];
    [ TransactionStreamBlocking ]   [ CoreClientBlocking ];
  )]
pub struct stream_type {
    pub client: client_type,
    pub network: String,
    pub from_state_version: u64,
    pub limit: u32,
    pub sbor_format_options: SborFormatOptions,
    pub transaction_format_options: TransactionFormatOptions,
    pub substate_format_options: SubstateFormatOptions,
    pub include_proofs: bool,
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ CoreClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ CoreClientBlocking ] [ must_be_sync ];
  )]
impl stream_type {
    pub fn new(
        client: client_type,
        network: String,
        from_state_version: u64,
        limit: u32,
        sbor_format_options: SborFormatOptions,
        transaction_format_options: TransactionFormatOptions,
        substate_format_options: SubstateFormatOptions,
        include_proofs: bool,
    ) -> stream_type {
        if from_state_version == 0 {
            panic!("from_state_version must be greater than 0");
        }
        stream_type {
            client,
            network,
            from_state_version,
            limit,
            sbor_format_options,
            transaction_format_options,
            substate_format_options,
            include_proofs,
        }
    }

    #[maybe_async_attr]
    pub async fn next(
        &mut self,
    ) -> Result<TransactionStream200ResponseBody, CoreApiError> {
        let response = self
            .client
            .transaction_stream_request(
                self.network.clone(),
                self.from_state_version,
                self.limit,
                self.sbor_format_options.clone(),
                self.transaction_format_options.clone(),
                self.substate_format_options.clone(),
                self.include_proofs,
            )
            .await?;
        let last = response.transactions.last();
        if let Some(transaction) = last {
            self.from_state_version =
                transaction.resultant_state_identifiers.state_version + 1;
        }
        Ok(response)
    }
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ CoreClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ CoreClientBlocking ] [ must_be_sync ];
  )]
impl client_type {
    pub fn new_transaction_stream(
        &self,
        network: String,
        from_state_version: u64,
        limit: u32,
        sbor_format_options: SborFormatOptions,
        transaction_format_options: TransactionFormatOptions,
        substate_format_options: SubstateFormatOptions,
        include_proofs: bool,
    ) -> stream_type {
        stream_type::new(
            self.clone(),
            network,
            from_state_version,
            limit,
            sbor_format_options,
            transaction_format_options,
            substate_format_options,
            include_proofs,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction_stream() {
        // Test via the public core API provided by Grove
        let client = CoreClientBlocking::new(
            "https://radix-mainnet.rpc.grove.city/v1/326002fc/core".to_string(),
        );
        let mut stream = client.new_transaction_stream(
            "mainnet".to_string(),
            1,
            1,
            SborFormatOptions::default(),
            TransactionFormatOptions::default(),
            SubstateFormatOptions::default(),
            false,
        );
        let result = stream.next().unwrap();
        assert_eq!(result.transactions.len(), 1);
    }
}
