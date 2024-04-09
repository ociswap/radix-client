use crate::{GatewayClientAsync, GatewayClientBlocking};

#[derive(Debug, Clone)]
pub struct RequestBuilderAsync<'a, R> {
    pub client: &'a GatewayClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct RequestBuilderBlocking<'a, R> {
    pub client: &'a GatewayClientBlocking,
    pub request: R,
}

impl<R> RequestBuilderAsync<'_, R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}

impl<R> RequestBuilderBlocking<'_, R> {
    pub fn build(&self) -> &R {
        &self.request
    }
}
