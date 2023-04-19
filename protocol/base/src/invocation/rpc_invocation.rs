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
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use dashmap::DashMap;

use crate::invocation::{Invocation, InvocationType};
use crate::invoker::Invoker;

#[derive(Clone)]
pub struct RpcInvocationService {
    inner: RpcInvocation,
}


#[derive(Clone, Default)]
pub struct RpcInvocation {
    method_name: String,
    parameter_type_names: Vec<String>,
    parameter_values: Vec<InvocationType>,
    arguments: Vec<InvocationType>,
    reply: Option<Arc<dyn Any>>,
    invoker: Option<Arc<dyn Invoker<Output=dyn Any>>>,
    attachments: DashMap<String, InvocationType>,
    attributes: DashMap<String, InvocationType>,
}

impl Debug for RpcInvocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcInvocation")
            .field("method_name", &self.method_name)
            .finish()
    }
}

impl RpcInvocation {
    pub fn builder() -> RpcInvocation {
        RpcInvocation::default()
    }

    pub fn set_method_name(mut self, method_name: &str) -> Self {
        self.method_name = method_name.to_string();
        self
    }
}

impl Invocation for RpcInvocation {
    fn method_name(&self) -> String {
        self.method_name.clone()
    }

    fn parameter_type_names(&self) -> Vec<String> {
        self.parameter_type_names.clone()
    }

    fn parameter_values(&self) -> Vec<InvocationType> {
        self.parameter_values.clone()
    }

    fn arguments(&self) -> Vec<InvocationType> {
        self.arguments.clone()
    }

    fn reply(&self) -> Option<Arc<dyn Any>> {
        self.reply.clone()
    }

    fn invoker(&self) -> Option<Arc<dyn Invoker<Output=dyn Any>>> {
        self.invoker.clone()
    }

    fn attachments(&self) -> DashMap<String, InvocationType> {
        self.attachments.clone()
    }

    fn get_attachment(&self, key: &str) -> Option<InvocationType> {
        let option = self.attachments.get(key);
        match option {
            None => None,
            Some(_) => Some(option.unwrap().clone()),
        }
    }

    fn set_attachment(&mut self, key: &str, value: InvocationType) {
        self.attachments.insert(key.to_string(), value);
    }

    fn attributes(&self) -> DashMap<String, InvocationType> {
        self.attributes.clone()
    }

    fn get_attribute(&self, key: &str) -> Option<InvocationType> {
        if self.attributes.contains_key(key) {
            Some(self.attributes.get(key).unwrap().clone())
        } else {
            None
        }
    }

    fn get_attribute_with_default(&self, key: &str, default: InvocationType) -> InvocationType {
        if self.attributes.contains_key(key) {
            self.attributes.get(key).unwrap().clone()
        } else {
            default
        }
    }

    fn set_attribute(&mut self, key: &str, value: InvocationType) {
        self.attributes.insert(key.to_string(), value);
    }
}
