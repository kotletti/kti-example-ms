use std::error::Error;

use async_trait::async_trait;

use crate::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoCommandRepository: Send + Sync {
  async fn create(&self, todo: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>>;
  async fn update(&self, todo: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>>;
  async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
