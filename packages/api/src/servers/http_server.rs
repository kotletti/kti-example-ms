use std::{error::Error, net::SocketAddr};

use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::http::HealthHttpController;

pub struct HttpServer {
  address: String,
}

impl HttpServer {
  pub fn new(address: &str) -> Self {
    HttpServer {
      address: address.to_string(),
    }
  }

  pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
    let socket_address: SocketAddr = self
      .address
      .parse()
      .expect("Unable to parse the socket address.");

    println!("Todo http server started at: http://{}", &self.address);

    let listener = TcpListener::bind(socket_address).await.unwrap();

    loop {
      let (tcp, _) = listener.accept().await.unwrap();

      let io = TokioIo::new(tcp);

      tokio::spawn(async move {
        if let Err(err) = http1::Builder::new()
          .serve_connection(io, service_fn(HealthHttpController::router))
          .await
        {
          println!("Error serving connection: {:?}", err);
        }
      });
    }
  }
}
