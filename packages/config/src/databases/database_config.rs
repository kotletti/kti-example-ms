use std::env;

pub struct DatabaseConfig {
  database_url: String,
}

impl DatabaseConfig {
  pub fn new() -> Self {
    dotenvy::dotenv().unwrap();

    let database_url = match env::var("DATABASE_URL") {
      Ok(r) => r.into(),
      _ => panic!("Database URL is undefined."),
    };

    Self { database_url }
  }

  pub fn get_database_url(&self) -> &str {
    &self.database_url
  }
}
