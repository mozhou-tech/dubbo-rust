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

use axum::{routing::get, Router, ServiceExt};
use proto::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use std::net::SocketAddr;
use hyper::{header};
use hyper::header::HeaderValue;
use tonic::{Response as TonicResponse, Status};
use tower::{ServiceBuilder};
use tower_http::set_header::{SetRequestHeaderLayer, SetResponseHeaderLayer};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use dubbo_logger::tracing::info;
use crate::multiplex_service::MultiplexService;

mod proto {
    tonic::include_proto!("helloworld");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("helloworld_descriptor");
}

#[derive(Default)]
pub(crate) struct GrpcServiceImpl {}

#[tonic::async_trait]
impl Greeter for GrpcServiceImpl {
    async fn say_hello(
        &self,
        request: tonic::Request<HelloRequest>,
    ) -> Result<TonicResponse<HelloReply>, Status> {
        info!("Got a request from {:?}", request.remote_addr());

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(TonicResponse::new(reply))
    }
}

pub(crate) async fn web_root() -> &'static str {
    info!("rest request received");
    "Hello, World!"
}

pub async fn launch() {
    // build the rest invocation
    let rest = Router::new().route("/", get(web_root));
    // build the grpc invocation
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();
    let grpc = tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(GreeterServer::new(GrpcServiceImpl::default()))
        .into_service();

    // combine them into one invocation
    let service = MultiplexService::new(rest, grpc);
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::default());
    let svc = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(
            SetRequestHeaderLayer::if_not_present(
                header::WARNING,
                HeaderValue::from_static("my very cool app"),
            )
        )
        .layer(
            SetResponseHeaderLayer::overriding(
                header::USER_AGENT,
                HeaderValue::from_static("My-Value"),
            )
        )
        .layer(trace_layer)
        .service(service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(svc.into_make_service())
        .await
        .unwrap();
}
