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
use dubbo_base::typed_value::TypedValue;

use crate::invocation::{Invocation};
use crate::invoker::Invoker;

#[derive(Clone)]
pub struct RpcInvocationService {
    inner: RpcInvocation,
}


#[derive(Clone, Default)]
pub struct RpcInvocation {
    method_name: String,
    parameter_type_names: Vec<String>,
    parameter_values: Vec<TypedValue>,
    arguments: Vec<TypedValue>,
    reply: Option<Arc<dyn Any>>,
    invoker: Option<Arc<dyn Invoker<Output=dyn Any>>>,
    attachments: DashMap<String, TypedValue>,
    attributes: DashMap<String, TypedValue>,
}

impl Debug for RpcInvocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcInvocation")
            .field("method_name", &self.method_name)
            .field("arguments", &self.arguments)
            .field("parameter_values", &self.parameter_values)
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

    pub fn set_parameter_type_names(mut self, parameter_type_names: Vec<String>) -> Self {
        self.parameter_type_names = parameter_type_names;
        self
    }

    pub fn set_parameter_values(mut self, parameter_values: Vec<TypedValue>) -> Self {
        self.parameter_values = parameter_values;
        self
    }
    pub fn set_arguments(mut self, arguments: Vec<TypedValue>) -> Self {
        self.arguments = arguments;
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

    fn parameter_values(&self) -> Vec<TypedValue> {
        self.parameter_values.clone()
    }

    fn arguments(&self) -> Vec<TypedValue> {
        self.arguments.clone()
    }

    fn reply(&self) -> Option<Arc<dyn Any>> {
        self.reply.clone()
    }

    fn invoker(&self) -> Option<Arc<dyn Invoker<Output=dyn Any>>> {
        self.invoker.clone()
    }

    fn attachments(&self) -> DashMap<String, TypedValue> {
        self.attachments.clone()
    }

    fn get_attachment(&self, key: &str) -> Option<TypedValue> {
        let option = self.attachments.get(key);
        match option {
            None => None,
            Some(_) => Some(option.unwrap().clone()),
        }
    }

    fn set_attachment(&mut self, key: &str, value: TypedValue) {
        self.attachments.insert(key.to_string(), value);
    }

    fn attributes(&self) -> DashMap<String, TypedValue> {
        self.attributes.clone()
    }

    fn get_attribute(&self, key: &str) -> Option<TypedValue> {
        if self.attributes.contains_key(key) {
            Some(self.attributes.get(key).unwrap().clone())
        } else {
            None
        }
    }

    fn get_attribute_with_default(&self, key: &str, default: TypedValue) -> TypedValue {
        if self.attributes.contains_key(key) {
            self.attributes.get(key).unwrap().clone()
        } else {
            default
        }
    }

    fn set_attribute(&mut self, key: &str, value: TypedValue) {
        self.attributes.insert(key.to_string(), value);
    }
}
