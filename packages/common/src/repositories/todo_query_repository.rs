use std::error::Error;

use async_trait::async_trait;

use crate::entities::todo_entity::TodoEntity;

#[async_trait]
pub trait TodoQueryRepository: Send + Sync {
  async fn get_by_id(&self, id: &str) -> Result<Option<TodoEntity>, Box<dyn Error>>;

  async fn get_by_completed(&self, completed: bool) -> Result<Vec<TodoEntity>, Box<dyn Error>>;

  async fn get_paginated(&self, page: i64, limit: i64) -> Result<Vec<TodoEntity>, Box<dyn Error>>;
}
