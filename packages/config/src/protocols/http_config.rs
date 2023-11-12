use std::env;

pub struct HttpConfig {
  address: String,
}

impl HttpConfig {
  pub fn new() -> Self {
    dotenvy::dotenv().unwrap();

    let address = match env::var("HTTP_ADDRESS") {
      Ok(r) => r.into(),
      _ => panic!("HTTP address is undefined."),
    };

    Self { address }
  }

  pub fn get_address(&self) -> &str {
    &self.address
  }
}
