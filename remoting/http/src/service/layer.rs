use std::fmt;
use std::task::{Context, Poll};
use tower::Layer;
use tower_service::Service;
use dubbo_logger::tracing::debug;

pub struct LogLayer {
    target: &'static str,
}

impl LogLayer{
    pub fn new(target:&'static str)->LogLayer{
        LogLayer{
            target
        }
    }
}

impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            target: self.target,
            service
        }
    }
}

#[derive(Clone)]
pub struct LogService<S> {
    target: &'static str,
    service: S,
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
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // Insert log statement here or other functionality
        debug!("request = {:?}, target = {:?}", request, self.target);
        self.service.call(request)
    }
}