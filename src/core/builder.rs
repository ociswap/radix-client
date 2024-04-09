use crate::{CoreClientAsync, CoreClientBlocking};

#[derive(Debug, Clone)]
pub struct AsyncRequestBuilder<'a, R> {
    pub client: &'a CoreClientAsync,
    pub request: R,
}

#[derive(Debug, Clone)]
pub struct BlockingRequestBuilder<'a, R> {
    pub client: &'a CoreClientBlocking,
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
