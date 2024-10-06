use crate::core::builder::RequestBuilderAsync;
use crate::core::builder::RequestBuilderBlocking;
use crate::core::error::CoreApiError;
use crate::core::models::*;
use crate::CoreClientAsync;
use crate::CoreClientBlocking;
use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};

/// A managed stream client that starts at a specific state version and fetches transactions
/// in chronological order. It allows for easy fetching of the next page with `.next()`.
/// It should never fetch the same transaction twice. It's designed to be able to handle
/// the case where the pagination catches up with the current state version. In that case,
/// the amount of items in the response will simply be 0.
#[duplicate_item(
    stream_type                     builder_type ;
    [ TransactionStreamAsync ]      [ RequestBuilderAsync ];
    [ TransactionStreamBlocking ]   [ RequestBuilderBlocking ];
)]
#[derive(Debug)]
pub struct stream_type {
    pub cursor: Option<String>,
    pub builder: builder_type<GetCommittedTransactionsRequest>,
    pub last_seen_state_version: u64,
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ CoreClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ CoreClientBlocking ] [ must_be_sync ];
)]
impl stream_type {
    pub fn new(
        client: &client_type,
        network: String,
        from_state_version: u64,
        limit_per_page: u32,
        transaction_format_options: TransactionFormatOptions,
        sbor_format_options: SborFormatOptions,
    ) -> stream_type {
        if from_state_version == 0 {
            panic!("from_state_version must be greater than 0");
        }
        let builder = client
            .committed_transactions_builder()
            .network(network)
            .transaction_format_options(transaction_format_options)
            .sbor_format_options(sbor_format_options)
            .limit(limit_per_page)
            .from_state_version(from_state_version)
            .clone();
        stream_type {
            cursor: None,
            builder,
            last_seen_state_version: from_state_version - 1,
        }
    }

    #[maybe_async_attr]
    pub async fn next(
        &mut self,
    ) -> Result<GetCommittedTransactions200ResponseBody, CoreApiError> {
        let mut response = self.builder.fetch().await?;

        response.transactions = response
            .transactions
            .into_iter()
            .filter(|transaction| {
                transaction.resultant_state_identifiers.state_version
                    > self.last_seen_state_version
            })
            .collect();

        let last = response.transactions.last();
        if let Some(transaction) = last {
            self.builder.from_state_version(
                transaction.resultant_state_identifiers.state_version + 1,
            );
            self.last_seen_state_version =
                transaction.resultant_state_identifiers.state_version;
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
        limit_per_page: u32,
        transaction_format_options: TransactionFormatOptions,
        sbor_format_options: SborFormatOptions,
    ) -> stream_type {
        stream_type::new(
            &self,
            network,
            from_state_version,
            limit_per_page,
            transaction_format_options,
            sbor_format_options,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::PUBLIC_CORE_URL;

    use super::*;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn test_30_transactions() {
        let client = CoreClientBlocking::new(PUBLIC_CORE_URL.to_string());
        let mut stream = client.new_transaction_stream(
            "mainnet".to_string(),
            1000000,
            3,
            TransactionFormatOptions {
                balance_changes: true,
                ..Default::default()
            },
            Default::default(),
        );

        let mut count = 0;
        for _ in 0..10 {
            let response = stream.next().unwrap();
            if response.transactions.len() == 0 {
                sleep(Duration::from_secs(1));
                continue;
            }
            count += response.transactions.len();
            println!(
                "State version: {}",
                response.transactions[0]
                    .resultant_state_identifiers
                    .state_version
            );
        }
        assert_eq!(count, 30);
    }
}
