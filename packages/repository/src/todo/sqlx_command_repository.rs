use std::error::Error;

use async_trait::async_trait;

use sqlx::{types::Uuid, PgPool};
use todo_common::{
  entities::todo_entity::TodoEntity, repositories::todo_command_repository::TodoCommandRepository,
};

use crate::{mappers::todo_sqlx_mapper::TodoSqlxMapper, utils::sqlx_parse_utils::sqlx_parse_utils};

#[derive(Clone)]
pub struct SqlxCommandRepository {
  pool: PgPool,
}

impl SqlxCommandRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl TodoCommandRepository for SqlxCommandRepository {
  async fn create(&self, todo: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>> {
    let created_at = match sqlx_parse_utils::string_to_timestamp(&todo.get_created_at()) {
      Ok(r) => r,
      Err(e) => return Err(e.into()),
    };

    let updated_at = match sqlx_parse_utils::string_to_timestamp(&todo.get_updated_at()) {
      Ok(r) => r,
      Err(e) => return Err(e.into()),
    };

    let row = sqlx::query(
      "
      INSERT INTO todo (id, name, description, completed, created_at, updated_at)
      VALUES ($1, $2, $3, $4, $5, $6) RETURNING *
    ",
    )
    .bind(Uuid::parse_str(&todo.get_id()).unwrap())
    .bind(&todo.get_name())
    .bind(&todo.get_description())
    .bind(&todo.get_completed())
    .bind(created_at)
    .bind(updated_at)
    .fetch_one(&self.pool)
    .await
    .unwrap();

    Ok(TodoSqlxMapper::pg_row_to_entity(&row))
  }

  async fn update(&self, todo: &TodoEntity) -> Result<TodoEntity, Box<dyn Error>> {
    let updated_at = match sqlx_parse_utils::string_to_timestamp(&todo.get_updated_at()) {
      Ok(r) => r,
      Err(e) => return Err(e.into()),
    };

    let row = sqlx::query(
      "
        UPDATE 
          todo 
        SET 
          name = $1, 
          description = $2, 
          completed = $3, 
          updated_at = $4 
        WHERE 
          id = $5 
        RETURNING *
      ",
    )
    .bind(&todo.get_name())
    .bind(&todo.get_description())
    .bind(&todo.get_completed())
    .bind(updated_at)
    .bind(Uuid::parse_str(&todo.get_id()).unwrap())
    .fetch_one(&self.pool)
    .await
    .unwrap();

    Ok(TodoSqlxMapper::pg_row_to_entity(&row))
  }

  async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
    sqlx::query(
      "
        DELETE FROM todo WHERE id = $1
      ",
    )
    .bind(Uuid::parse_str(id).unwrap())
    .execute(&self.pool)
    .await
    .unwrap();

    Ok(())
  }
}
