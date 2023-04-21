use std::fmt;
use std::task::{Context, Poll};
use tower_service::Service;
use dubbo_logger::tracing::debug;

#[derive(Clone)]
pub struct LogService<S> {
    pub(crate) target: &'static str,
    pub(crate) inner: S,
}

impl<S, Request> Service<Request> for LogService<S>
    where
        S: Service<Request>,
        Request: fmt::Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        debug!("request = {:?}, target = {:?}", request, self.target);
        self.inner.call(request)
    }
}