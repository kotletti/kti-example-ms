pub mod database_module {
  use sqlx::postgres::PgPool;

  pub async fn create_postgres_pool(url: &str) -> PgPool {
    PgPool::connect(url).await.unwrap()
  }
}
