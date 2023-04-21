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
use std::task::{Context, Poll};
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::routing::IntoMakeService;
use axum::ServiceExt;
use hyper::Request;
use hyper::Body;
use tower_service::Service;
use protocol_base::invocation::RpcInvocation;
use crate::invocation::converter::HttpRequestWrapper;


#[derive(Clone)]
pub struct RpcInvocationService<S> {
    pub inner: S,
    pub invocation: RpcInvocation,
}

impl<S> Service<Request<Body>> for RpcInvocationService<S>
    where
        S: Service<RpcInvocation> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;
    // q? 解释下后面的代码

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let rpc_invocation = RpcInvocation::from(HttpRequestWrapper::new(request));
        self.inner.call(rpc_invocation)
    }
}
