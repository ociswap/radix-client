use crate::gateway::builder::RequestBuilderAsync;
use crate::gateway::builder::RequestBuilderBlocking;
use crate::gateway::error::GatewayApiError;
use crate::gateway::models::*;
use crate::GatewayClientAsync;
use crate::GatewayClientBlocking;
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
    pub builder: builder_type<TransactionStreamRequestBody>,
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ GatewayClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ GatewayClientBlocking ] [ must_be_sync ];
)]
impl stream_type {
    pub fn new(
        client: &client_type,
        from_state_version: u64,
        limit_per_page: u32,
    ) -> stream_type {
        if from_state_version == 0 {
            panic!("from_state_version must be greater than 0");
        }
        let builder = client
            .transactions_stream_builder()
            .from_state_version(from_state_version)
            .order(Order::Asc)
            .kind_filter(TransactionKindFilter::User)
            .limit_per_page(limit_per_page)
            .with_receipt_events()
            .clone();
        stream_type {
            cursor: None,
            builder,
        }
    }

    #[maybe_async_attr]
    pub async fn next(
        &mut self,
    ) -> Result<TransactionStream200ResponseBody, GatewayApiError> {
        let response = self.builder.fetch().await?;

        let last = response.items.last();
        if let Some(transaction) = last {
            self.builder
                .from_state_version(transaction.state_version + 1);
        }
        Ok(response)
    }
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ GatewayClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ GatewayClientBlocking ] [ must_be_sync ];
)]
impl client_type {
    pub fn new_transaction_stream(
        &self,
        from_state_version: u64,
        limit_per_page: u32,
    ) -> stream_type {
        stream_type::new(&self, from_state_version, limit_per_page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread::sleep, time::Duration};

    #[test]
    fn test_30_transactions() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let mut stream = client.new_transaction_stream(1, 3);

        let mut count = 0;
        for _ in 0..10 {
            let response = stream.next().unwrap();
            if response.items.len() == 0 {
                sleep(Duration::from_secs(1));
                continue;
            }
            count += response.items.len();
            println!("State version: {}", response.items[0].state_version);
        }
        assert_eq!(count, 30);
    }
}
