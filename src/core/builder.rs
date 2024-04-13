use crate::{CoreClientAsync, CoreClientBlocking};

#[derive(Debug, Clone)]
pub struct RequestBuilderAsync<'a, R> {
    pub client: &'a CoreClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct RequestBuilderBlocking<'a, R> {
    pub client: &'a CoreClientBlocking,
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
