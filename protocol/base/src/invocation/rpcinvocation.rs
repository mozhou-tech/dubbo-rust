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
use std::any::Any;
use std::sync::Arc;
use std::task::{Context, Poll};

use dashmap::DashMap;
use tower_service::Service;

use crate::invocation::{Invocation, InvocationField};
use crate::invoker::Invoker;

#[derive(Clone)]
pub struct RpcInvocationService {
    inner: RpcInvocation,
}

// impl<Request> Service<Request> for RpcInvocationService {
//     type Response = ();
//     type Error = ();
//     type Future = ();
//
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         todo!()
//     }
//
//     fn call(&mut self, req: Request) -> Self::Future {
//         todo!()
//     }
// }

#[derive(Clone)]
pub struct RpcInvocation {
    method_name: String,
    parameter_type_names: Vec<String>,
    parameter_values: Vec<InvocationField>,
    arguments: Vec<InvocationField>,
    reply: Arc<dyn Any>,
    invoker: Arc<dyn Invoker<Output = dyn Any>>,
    attachments: DashMap<String, InvocationField>,
    attributes: DashMap<String, InvocationField>,
}

impl Invocation for RpcInvocation {
    fn method_name(&self) -> String {
        self.method_name.clone()
    }

    fn parameter_type_names(&self) -> Vec<String> {
        self.parameter_type_names.clone()
    }

    fn parameter_values(&self) -> Vec<InvocationField> {
        self.parameter_values.clone()
    }

    fn arguments(&self) -> Vec<InvocationField> {
        self.arguments.clone()
    }

    fn reply(&self) -> Arc<dyn Any> {
        self.reply.clone()
    }

    fn invoker(&self) -> Arc<dyn Invoker<Output = dyn Any>> {
        self.invoker.clone()
    }

    fn attachments(&self) -> DashMap<String, InvocationField> {
        self.attachments.clone()
    }

    fn get_attachment(&self, key: &str) -> Option<InvocationField> {
        let option = self.attachments.get(key);
        match option {
            None => None,
            Some(_) => Some(option.unwrap().clone()),
        }
    }

    fn set_attachment(&mut self, key: &str, value: InvocationField) {
        self.attachments.insert(key.to_string(), value);
    }

    fn attributes(&self) -> DashMap<String, InvocationField> {
        self.attributes.clone()
    }

    fn get_attribute(&self, key: &str) -> Option<InvocationField> {
        if self.attributes.contains_key(key) {
            Some(self.attributes.get(key).unwrap().clone())
        } else {
            None
        }
    }

    fn get_attribute_with_default(&self, key: &str, default: InvocationField) -> InvocationField {
        if self.attributes.contains_key(key) {
            self.attributes.get(key).unwrap().clone()
        } else {
            default
        }
    }

    fn set_attribute(&mut self, key: &str, value: InvocationField) {
        self.attributes.insert(key.to_string(), value);
    }
}
