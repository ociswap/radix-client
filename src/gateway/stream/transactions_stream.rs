use self::gateway::{
    builder::{RequestBuilderAsync, RequestBuilderBlocking},
    error::GatewayApiError,
    match_response,
    models::*,
};
use crate::*;
use chrono::Utc;
use duplicate::duplicate_item;
use maybe_async::*;

#[duplicate_item(
    client_type                 maybe_async_attr ;
    [ GatewayClientAsync ]     [ must_be_async ];
    [ GatewayClientBlocking ]  [ must_be_sync ];
  )]
impl client_type {
    #[maybe_async_attr]
    pub async fn transactions_stream(
        &self,
        request: TransactionStreamRequestBody,
    ) -> Result<TransactionStream200ResponseBody, GatewayApiError> {
        let (text, status) = self.post("stream/transactions", request).await?;
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
    pub fn transactions_stream_builder(
        &self,
    ) -> request_type<TransactionStreamRequestBody> {
        let request = TransactionStreamRequestBody {
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
impl builder_type<'_, TransactionStreamRequestBody> {
    pub fn affected_global_entities_filter(
        &mut self,
        value: Vec<String>,
    ) -> &mut Self {
        self.request.affected_global_entities_filter = Some(value);
        self
    }

    pub fn manifest_accounts_deposited_into_filter(
        &mut self,
        value: Vec<String>,
    ) -> &mut Self {
        self.request.manifest_accounts_deposited_into_filter = Some(value);
        self
    }

    pub fn manifest_accounts_withdrawn_from_filter(
        &mut self,
        value: Vec<String>,
    ) -> &mut Self {
        self.request.manifest_accounts_withdrawn_from_filter = Some(value);
        self
    }

    pub fn manifest_resources_filter(
        &mut self,
        value: Vec<String>,
    ) -> &mut Self {
        self.request.manifest_resources_filter = Some(value);
        self
    }

    // pub struct LedgerStateSelector {
    //     pub state_version: Option<u64>,
    //     pub timestamp: Option<u64>,
    //     pub epoch: Option<u64>,
    //     pub round: Option<u64>,
    // }

    pub fn at_state_version(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            state_version: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn at_timestamp(&mut self, value: chrono::DateTime<Utc>) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            timestamp: Some(value.timestamp() as u64),
            ..Default::default()
        });
        self
    }

    pub fn at_epoch(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            epoch: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn at_round(&mut self, value: u64) -> &mut Self {
        self.request.at_ledger_state = Some(LedgerStateSelector {
            round: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn from_state_version(&mut self, value: u64) -> &mut Self {
        self.request.from_ledger_state = Some(LedgerStateSelector {
            state_version: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn from_timestamp(
        &mut self,
        value: chrono::DateTime<Utc>,
    ) -> &mut Self {
        self.request.from_ledger_state = Some(LedgerStateSelector {
            timestamp: Some(value.timestamp() as u64),
            ..Default::default()
        });
        self
    }

    pub fn from_epoch(&mut self, value: u64) -> &mut Self {
        self.request.from_ledger_state = Some(LedgerStateSelector {
            epoch: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn from_round(&mut self, value: u64) -> &mut Self {
        self.request.from_ledger_state = Some(LedgerStateSelector {
            round: Some(value),
            ..Default::default()
        });
        self
    }

    pub fn cursor(&mut self, value: String) -> &mut Self {
        self.request.cursor = Some(value);
        self
    }

    pub fn limit_per_page(&mut self, value: u32) -> &mut Self {
        self.request.limit_per_page = Some(value);
        self
    }

    // #[derive(Serialize, Deserialize, Debug, Clone, Default)]
    // pub struct TransactionStreamOptIns {
    //     pub raw_hex: bool,
    //     pub receipt_state_changes: bool,
    //     pub receipt_fee_summary: bool,
    //     pub receipt_fee_source: bool,
    //     pub receipt_fee_destination: bool,
    //     pub receipt_costing_parameters: bool,
    //     pub receipt_events: bool,
    //     pub receipt_output: bool,
    //     pub affected_global_entities: bool,
    //     pub manifest_instructions: bool,
    //     pub balance_changes: bool,
    // }

    /// Helper function to update the opt_ins field
    /// with less boilerplate code in each setter.
    fn update_opt_ins(
        &mut self,
        function: fn(TransactionStreamOptIns) -> TransactionStreamOptIns,
    ) -> &mut Self {
        self.request.opt_ins = self
            .request
            .opt_ins
            .clone()
            .or(Some(TransactionStreamOptIns::default()))
            .map(function);
        self
    }

    pub fn with_raw_hex(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.raw_hex = true;
            opt_ins
        })
    }

    pub fn with_receipt_state_changes(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_state_changes = true;
            opt_ins
        })
    }

    pub fn with_receipt_fee_summary(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_fee_summary = true;
            opt_ins
        })
    }

    pub fn with_receipt_fee_source(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_fee_source = true;
            opt_ins
        })
    }

    pub fn with_receipt_fee_destination(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_fee_destination = true;
            opt_ins
        })
    }

    pub fn with_receipt_costing_parameters(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_costing_parameters = true;
            opt_ins
        })
    }

    pub fn with_receipt_events(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_events = true;
            opt_ins
        })
    }

    pub fn with_receipt_output(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.receipt_output = true;
            opt_ins
        })
    }

    pub fn with_affected_global_entities(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.affected_global_entities = true;
            opt_ins
        })
    }

    pub fn with_manifest_instructions(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.manifest_instructions = true;
            opt_ins
        })
    }

    pub fn with_balance_changes(&mut self) -> &mut Self {
        self.update_opt_ins(|mut opt_ins| {
            opt_ins.balance_changes = true;
            opt_ins
        })
    }

    pub fn order(&mut self, value: Order) -> &mut Self {
        self.request.order = Some(value);
        self
    }

    pub fn kind_filter(&mut self, value: TransactionKindFilter) -> &mut Self {
        self.request.kind_filter = Some(value);
        self
    }

    #[maybe_async_attr]
    pub async fn fetch(
        &self,
    ) -> Result<TransactionStream200ResponseBody, GatewayApiError> {
        self.client.transactions_stream(self.request.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{constants::PUBLIC_GATEWAY_URL, GatewayClientBlocking};

    #[test]
    fn simple_stream_request() {
        let client = GatewayClientBlocking::new(PUBLIC_GATEWAY_URL.to_string());
        let response = client
            .transactions_stream_builder()
            .order(crate::gateway::stream::transactions_stream::Order::Asc)
            .limit_per_page(1)
            .with_raw_hex()
            .fetch()
            .unwrap();
        println!("{response:?}");
    }
}
