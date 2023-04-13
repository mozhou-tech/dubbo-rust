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

use dashmap::DashMap;
use std::collections::HashMap;
use std::{any::Any, sync::Arc};
use tokio::sync::Mutex;

use crate::invoker::Invoker;

#[derive(Clone, Debug)]
pub enum InvocationField {
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
    fn parameter_values(&self) -> Vec<InvocationField>;
    fn arguments(&self) -> Vec<InvocationField>;
    fn reply(&self) -> Arc<dyn Any>;
    fn invoker(&self) -> Arc<dyn Invoker<Output = dyn Any>>;
    fn attachments(&self) -> DashMap<String, InvocationField>;
    fn get_attachment(&self, key: &str) -> Option<InvocationField>;
    fn set_attachment(&mut self, key: &str, value: InvocationField);
    fn attributes(&self) -> DashMap<String, InvocationField>;
    fn get_attribute(&self, key: &str) -> Option<InvocationField>;
    fn get_attribute_with_default(&self, key: &str, default: InvocationField) -> InvocationField;
    fn set_attribute(&mut self, key: &str, value: InvocationField);
}

pub type BoxInvocation = Arc<dyn Invocation + Send + Sync>;
