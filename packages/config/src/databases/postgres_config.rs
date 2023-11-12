use std::env;

pub struct PostgresConfig {
  host: String,
  port: i16,
  username: String,
  password: String,
}

impl PostgresConfig {
  pub fn new() -> Self {
    dotenvy::dotenv().unwrap();

    let host = env::var("POSTGRES_HOST").unwrap_or("localhost".into());

    let port = match env::var("POSTGRES_PORT") {
      Ok(r) => r.parse::<i16>().unwrap(),
      _ => 5432,
    };

    let username = env::var("POSTGRES_USERNAME").unwrap_or("postgres".into());

    let password = env::var("POSTGRES_PASSWORD").unwrap_or("postgres".into());

    PostgresConfig {
      host,
      port,
      username,
      password,
    }
  }

  pub fn get_host(&self) -> &str {
    &self.host
  }

  pub fn get_port(&self) -> i16 {
    self.port
  }

  pub fn get_username(&self) -> &str {
    &self.username
  }

  pub fn get_password(&self) -> &str {
    &self.password
  }
}
