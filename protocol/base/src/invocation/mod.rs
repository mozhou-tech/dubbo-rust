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

use crate::invoker::Invoker;

#[derive(Clone, Debug)]
pub enum InvocationType {
    String(String),
    Bytes(Vec<u8>),
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
}

pub trait Invocation {
    fn method_name(&self) -> String;
    fn parameter_type_names(&self) -> Vec<String>;
    fn parameter_values(&self) -> Vec<InvocationType>;
    fn arguments(&self) -> Vec<InvocationType>;
    fn reply(&self) -> Option<Arc<dyn Any>>;
    fn invoker(&self) -> Option<Arc<dyn Invoker<Output=dyn Any>>>;
    fn attachments(&self) -> DashMap<String, InvocationType>;
    fn get_attachment(&self, key: &str) -> Option<InvocationType>;
    fn set_attachment(&mut self, key: &str, value: InvocationType);
    fn attributes(&self) -> DashMap<String, InvocationType>;
    fn get_attribute(&self, key: &str) -> Option<InvocationType>;
    fn get_attribute_with_default(&self, key: &str, default: InvocationType) -> InvocationType;
    fn set_attribute(&mut self, key: &str, value: InvocationType);
}

pub type BoxInvocation = Arc<dyn Invocation + Send + Sync>;
