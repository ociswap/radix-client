use crate::{CoreClientAsync, CoreClientBlocking};

#[derive(Debug, Clone)]
pub struct RequestBuilderAsync<R> {
    pub client: CoreClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct RequestBuilderBlocking<R> {
    pub client: CoreClientBlocking,
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
