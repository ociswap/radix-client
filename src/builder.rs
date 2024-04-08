use crate::{GatewayClientAsync, GatewayClientBlocking};

#[derive(Debug, Clone)]
pub struct AsyncRequestBuilder<'a, R> {
    pub client: &'a GatewayClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct BlockingRequestBuilder<'a, R> {
    pub client: &'a GatewayClientBlocking,
    pub request: R,
}

impl<R> AsyncRequestBuilder<'_, R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}

impl<R> BlockingRequestBuilder<'_, R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}
