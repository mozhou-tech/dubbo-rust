use hyper::{Body, Request};
use crate::invocation::RpcInvocation;

struct Wrapper(Request<Body>);

impl Wrapper {
    pub fn new(req: Request<Body>) -> Wrapper {
        Wrapper(req)
    }
}

impl From<Wrapper> for RpcInvocation {
    fn from(req: Wrapper) -> Self {
        RpcInvocation::builder()
            .set_method_name(req.0.uri().to_string().as_str())
    }
}


#[cfg(test)]
mod tests {
    use hyper::{Body, Method, Request};
    use crate::invocation::converter::Wrapper;
    use crate::invocation::RpcInvocation;

    #[test]
    fn test_from() {
        let req = Request::builder()
            .method(Method::POST)
            .uri("https://www.example.com/form")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Body::from("name=John+Doe&age=25")).unwrap();
        let wrapper = Wrapper::new(req);
        let invocation = RpcInvocation::from(wrapper);
        println!("{:?}", invocation);
    }
}