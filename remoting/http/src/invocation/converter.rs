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
use hyper::{Body, Request};
use protocol_base::invocation::RpcInvocation;

pub(crate) struct HttpRequestWrapper(Request<Body>);

impl HttpRequestWrapper {
    pub fn new(req: Request<Body>) -> HttpRequestWrapper {
        HttpRequestWrapper(req)
    }
}

impl From<HttpRequestWrapper> for RpcInvocation {
    fn from(req: HttpRequestWrapper) -> Self {
        RpcInvocation::builder()
            .set_method_name(req.0.uri().to_string().as_str())
    }
}


#[cfg(test)]
mod tests {
    use hyper::{Body, Method, Request};
    use protocol_base::invocation::RpcInvocation;
    use crate::invocation::converter::HttpRequestWrapper;

    #[test]
    fn test_from() {
        let req = Request::builder()
            .method(Method::POST)
            .uri("https://www.example.com/form")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("name=John+Doe&age=25")).unwrap();
        let wrapper = HttpRequestWrapper::new(req);
        let invocation = RpcInvocation::from(wrapper);
        println!("{:?}", invocation);
    }
}