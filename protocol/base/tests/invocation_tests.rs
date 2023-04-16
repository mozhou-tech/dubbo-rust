use bytes::Bytes;
use tokio::net::TcpListener;
use tower::make::Shared;
use tower::{service_fn, ServiceBuilder};
use tower_service::Service;

#[tokio::main]
pub async fn invocation_consumer() {
    let addr = "[::1]:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    let make_service = Shared::new(service_fn(|_| async {
        Ok::<_, std::io::Error>(EchoService)
    }));

    let server = ServiceBuilder::new()
        .layer(tokio_tower::TcpServerLayer::new(make_service))
        .into_new_service();

    let mut incoming = listener.incoming();
    while let Some(socket) = incoming.next().await {
        let socket = socket.unwrap();
        let _ = tokio::spawn(async move {
            let service = server.new_service().await.unwrap();
            let mut framed = Framed::new(socket, LinesCodec::new());
            let mut service = TcpServer::new(service, framed);

            if let Err(err) = service.serve().await {
                eprintln!("Error: {}", err);
            }
        });
    }
}

#[derive(Clone)]
struct EchoService;

impl Service<Bytes> for EchoService {
    type Response = Bytes;
    type Error = std::io::Error;
    type Future = futures::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Bytes) -> Self::Future {
        futures::future::ready(Ok(req))
    }
}
