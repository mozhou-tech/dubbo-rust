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
#![feature(type_alias_impl_trait)]
use std::time::Duration;

pub mod client;
pub mod connect;
pub mod status;

const DEFAULT_STREAM_WINDOW_SIZE: u32 = 1024 * 1024 * 2; // 2MB
const DEFAULT_CONN_WINDOW_SIZE: u32 = 1024 * 1024 * 5; // 5MB
const DEFAULT_MAX_FRAME_SIZE: u32 = 1024 * 16; // 16KB
const DEFAULT_MAX_SEND_BUF_SIZE: usize = 1024 * 1024; // 1MB
const DEFAULT_KEEPALIVE_TIMEOUT_SECS: Duration = Duration::from_secs(20); // 20s
const DEFAULT_MAX_CONCURRENT_RESET_STREAMS: usize = 10;

/// Configuration for the underlying h2 connection.
#[derive(Debug, Clone, Copy)]
pub struct Http2Config {
    pub(crate) init_stream_window_size: u32,
    pub(crate) init_connection_window_size: u32,
    pub(crate) adaptive_window: bool,
    pub(crate) max_frame_size: u32,
    pub(crate) http2_keepalive_interval: Option<Duration>,
    pub(crate) http2_keepalive_timeout: Duration,
    pub(crate) http2_keepalive_while_idle: bool,
    pub(crate) max_concurrent_reset_streams: usize,
    pub(crate) retry_canceled_requests: bool,
    pub(crate) max_send_buf_size: usize,
    pub(crate) accept_http1: bool,
}

impl Default for Http2Config {
    fn default() -> Self {
        Self {
            init_stream_window_size: DEFAULT_STREAM_WINDOW_SIZE,
            init_connection_window_size: DEFAULT_CONN_WINDOW_SIZE,
            adaptive_window: false,
            max_frame_size: DEFAULT_MAX_FRAME_SIZE,
            http2_keepalive_interval: None,
            http2_keepalive_timeout: DEFAULT_KEEPALIVE_TIMEOUT_SECS,
            http2_keepalive_while_idle: false,
            max_concurrent_reset_streams: DEFAULT_MAX_CONCURRENT_RESET_STREAMS,
            max_send_buf_size: DEFAULT_MAX_SEND_BUF_SIZE,
            retry_canceled_requests: true,
            accept_http1: false,
        }
    }
}
