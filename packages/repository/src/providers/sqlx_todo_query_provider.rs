pub mod sqlx_todo_query_provider {
  use std::error::Error;

  use async_trait::async_trait;

  use sqlx::PgPool;
  use todo_common::{
    entities::todo_entity::TodoEntity, repositories::todo_query_repository::TodoQueryRepository,
  };

  use crate::todo::sqlx_query_repository::SqlxQueryRepository;

  pub const TOKEN_PROVIDER: &'static str = "SQLX_TODO_QUERY_REPOSITORY_PROVIDER";

  #[derive(Clone)]
  pub struct Provider {
    provider: &'static str,
    repository: SqlxQueryRepository,
  }

  impl Provider {
    pub fn new(pool: PgPool) -> Self {
      Self {
        provider: TOKEN_PROVIDER,
        repository: SqlxQueryRepository::new(pool),
      }
    }

    pub fn get_provider(&self) -> &'static str {
      self.provider
    }
  }

  #[async_trait]
  impl TodoQueryRepository for Provider {
    async fn get_by_id(&self, id: &str) -> Result<Option<TodoEntity>, Box<dyn Error>> {
      self.repository.get_by_id(id).await
    }

    async fn get_by_completed(&self, completed: bool) -> Result<Vec<TodoEntity>, Box<dyn Error>> {
      self.repository.get_by_completed(completed).await
    }

    async fn get_paginated(
      &self,
      page: i64,
      limit: i64,
    ) -> Result<Vec<TodoEntity>, Box<dyn Error>> {
      self.repository.get_paginated(page, limit).await
    }
  }
}
