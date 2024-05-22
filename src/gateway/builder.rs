use crate::{GatewayClientAsync, GatewayClientBlocking};

/// Request builders. These are specialized for async and blocking clients.
/// They store the client and the request body, and the builder setters
/// are implemented on them.

#[derive(Debug, Clone)]
pub struct RequestBuilderAsync<R> {
    pub client: GatewayClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct RequestBuilderBlocking<R> {
    pub client: GatewayClientBlocking,
    pub request: R,
}

impl<R> RequestBuilderAsync<R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}

impl<R> RequestBuilderBlocking<R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}
