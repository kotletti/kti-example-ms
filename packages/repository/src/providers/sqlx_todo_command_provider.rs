pub mod sqlx_todo_command_provider {
  use std::error::Error;

  use async_trait::async_trait;

  use sqlx::PgPool;
  use todo_common::{
    entities::todo_entity::TodoEntity, repositories::todo_command_repository::TodoCommandRepository,
  };

  use crate::todo::sqlx_command_repository::SqlxCommandRepository;

  pub const TOKEN_PROVIDER: &'static str = "SQLX_TODO_COMMAND_REPOSITORY_PROVIDER";

  #[derive(Clone)]
  pub struct Provider {
    provider: &'static str,
    repository: SqlxCommandRepository,
  }

  impl Provider {
    pub fn new(pool: PgPool) -> Self {
      Self {
        provider: TOKEN_PROVIDER,
        repository: SqlxCommandRepository::new(pool),
      }
    }

    pub fn get_provider(&self) -> &'static str {
      self.provider
    }
  }

  #[async_trait]
  impl TodoCommandRepository for Provider {
    async fn create(&self, entity: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>> {
      self.repository.create(entity).await
    }

    async fn update(&self, entity: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>> {
      self.repository.update(entity).await
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
      self.repository.delete(id).await
    }
  }
}
