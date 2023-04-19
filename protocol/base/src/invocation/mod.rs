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

mod rpc_invocation;
pub mod service;
pub mod converter;

pub use rpc_invocation::RpcInvocation;

use dashmap::DashMap;
use std::{any::Any, sync::Arc};
use dubbo_base::typed_value::TypedValue;

use crate::invoker::Invoker;


pub trait Invocation {
    fn method_name(&self) -> String;
    fn parameter_type_names(&self) -> Vec<String>;
    fn parameter_values(&self) -> Vec<TypedValue>;
    fn arguments(&self) -> Vec<TypedValue>;
    fn reply(&self) -> Option<Arc<dyn Any>>;
    fn invoker(&self) -> Option<Arc<dyn Invoker<Output=dyn Any>>>;
    fn attachments(&self) -> DashMap<String, TypedValue>;
    fn get_attachment(&self, key: &str) -> Option<TypedValue>;
    fn set_attachment(&mut self, key: &str, value: TypedValue);
    fn attributes(&self) -> DashMap<String, TypedValue>;
    fn get_attribute(&self, key: &str) -> Option<TypedValue>;
    fn get_attribute_with_default(&self, key: &str, default: TypedValue) -> TypedValue;
    fn set_attribute(&mut self, key: &str, value: TypedValue);
}

pub type BoxInvocation = Arc<dyn Invocation + Send + Sync>;
