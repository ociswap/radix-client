use super::models::*;
use crate::GatewayClientAsync;
use crate::GatewayClientBlocking;
use duplicate::duplicate_item;
use maybe_async::{must_be_async, must_be_sync};

#[duplicate_item(
    stream_type                     client_type ;
    [ TransactionStreamAsync ]      [ GatewayClientAsync ];
    [ TransactionStreamBlocking ]   [ GatewayClientBlocking ];
)]
#[derive(Debug)]
pub struct stream_type {
    pub client: client_type,
    pub cursor: Option<String>,
    pub params: TransactionStreamRequestBody,
}

#[duplicate_item(
    stream_type                         client_type           maybe_async_attr;
    [ TransactionStreamAsync ]         [ GatewayClientAsync ]    [ must_be_async ];
    [ TransactionStreamBlocking ]      [ GatewayClientBlocking ] [ must_be_sync ];
  )]
impl stream_type {
    pub fn new(
        client: client_type,
        params: TransactionStreamRequestBody,
    ) -> stream_type {
        if let Some(state_selector) = &params.at_ledger_state {
            if let Some(from_state_version) = &state_selector.state_version {
                if *from_state_version == 0 {
                    panic!("from_state_version must be greater than 0");
                }
            }
        }
        stream_type {
            client,
            params,
            cursor: None,
        }
    }

    #[maybe_async_attr]
    pub async fn next(
        &mut self,
    ) -> Result<TransactionStream200ResponseBody, GatewayApiError> {
        let cloned_params = self.params.clone();
        let response = self
            .client
            .transaction_stream_request(
                cloned_params.at_ledger_state,
                cloned_params.from_ledger_state,
                None,
                cloned_params.limit_per_page,
                cloned_params.kind_filter,
                cloned_params.manifest_accounts_withdrawn_from_filter,
                cloned_params.manifest_accounts_deposited_into_filter,
                cloned_params.manifest_resources_filter,
                cloned_params.affected_global_entities_filter,
                cloned_params.order,
                cloned_params.opt_ins,
            )
            .await?;
        let last = response.items.last();
        if let Some(transaction) = last {
            self.params.from_ledger_state = Some(LedgerStateSelector {
                state_version: Some(transaction.state_version + 1),
                ..Default::default()
            });
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
        params: TransactionStreamRequestBody,
    ) -> stream_type {
        stream_type::new(self.clone(), params)
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn test_30_transactions() {
        let client = GatewayClientBlocking::new(
            "https://mainnet.radixdlt.com".to_string(),
        );
        let mut stream = TransactionStreamBlocking::new(
            client,
            TransactionStreamRequestBody {
                from_ledger_state: Some(LedgerStateSelector {
                    state_version: Some(1),
                    ..Default::default()
                }),
                limit_per_page: Some(3),
                kind_filter: TransactionKindFilter::User,
                order: Some(Order::Asc),
                ..Default::default()
            },
        );

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
