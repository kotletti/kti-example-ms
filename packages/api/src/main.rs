use servers::{grpc_server::GrpcServer, http_server::HttpServer};
use todo_config::protocols::{GrpcConfig, HttpConfig};

mod grpc;
mod http;
mod servers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let http_config = HttpConfig::new();
  let grpc_config = GrpcConfig::new();

  let grpc_server = GrpcServer::new(&grpc_config.get_address());
  let http_server = HttpServer::new(&http_config.get_address());

  let grpc_server_future = tokio::spawn(async move {
    grpc_server.run().await.unwrap();
  });

  let http_server_future = tokio::spawn(async move {
    http_server.run().await.unwrap();
  });

  std::mem::drop(tokio::join!(grpc_server_future, http_server_future));

  Ok(())
}
