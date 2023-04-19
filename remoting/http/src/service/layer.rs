/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::fmt::Debug;
use std::marker::PhantomData;
use tower::{Layer, Service};
use dubbo_logger::tracing::info;

pub struct LogLayer<S, Req> {
    target: &'static str,
    _req: PhantomData<fn() -> Req>,
    _s: PhantomData<fn() -> S>,
}

impl<S, Req> Clone for LogLayer<S, Req> {
    fn clone(&self) -> Self {
        Self {
            target: self.target,
            _req: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<S, Req> LogLayer<S, Req> {
    pub fn new(target: &'static str) -> Self {
        Self {
            target,
            _req: PhantomData,
            _s: PhantomData,
        }
    }
}

impl<S, Req> Layer<S> for LogLayer<S, Req>
    where
        S: Service<Req>,
        S::Error: Debug,
        Req: Debug,
{
    type Service = LogService<S, Req>;

    fn layer(&self, service: S) -> Self::Service {
        LogService {
            inner: service,
            target: self.target,
            _req: PhantomData,
        }
    }
}

pub struct LogService<S, Req> {
    inner: S,
    target: &'static str,
    _req: PhantomData<fn() -> Req>,
}


impl<S, Req> Service<Req> for LogService<S, Req>
    where
        S: Service<Req>,
        S::Error: Debug,
        Req: Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        info!("{} {:?}", self.target, req);
        self.inner.call(req)
    }
}