use std::env;

pub struct GrpcConfig {
  address: String,
}

impl GrpcConfig {
  pub fn new() -> Self {
    dotenvy::dotenv().unwrap();

    let address = match env::var("GRPC_ADDRESS") {
      Ok(r) => r.into(),
      _ => panic!("GRPC address is undefined."),
    };

    Self { address }
  }

  pub fn get_address(&self) -> &str {
    &self.address
  }
}
