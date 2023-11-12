use std::error::Error;

use async_trait::async_trait;

use sqlx::{postgres::PgRow, types::Uuid, PgPool};
use todo_common::{
  entities::todo_entity::TodoEntity, repositories::todo_query_repository::TodoQueryRepository,
};

use crate::mappers::todo_sqlx_mapper::TodoSqlxMapper;

#[derive(Clone)]
pub struct SqlxQueryRepository {
  pool: PgPool,
}

impl SqlxQueryRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl TodoQueryRepository for SqlxQueryRepository {
  async fn get_by_id(&self, id: &str) -> Result<Option<TodoEntity>, Box<dyn Error>> {
    let row = sqlx::query(
      "
      SELECT * FROM todo WHERE id = $1 LIMIT 1
    ",
    )
    .bind(Uuid::parse_str(id).unwrap())
    .fetch_one(&self.pool)
    .await;

    match row {
      Ok(r) => Ok(Some(TodoSqlxMapper::pg_row_to_entity(&r))),
      Err(_) => return Err("Cant find todo by id".into()),
    }
  }

  async fn get_by_completed(&self, completed: bool) -> Result<Vec<TodoEntity>, Box<dyn Error>> {
    let todo_list = sqlx::query(
      "
      SELECT * FROM todo WHERE completed = $1
    ",
    )
    .bind(completed)
    .map(|i: PgRow| TodoSqlxMapper::pg_row_to_entity(&i))
    .fetch_all(&self.pool)
    .await
    .unwrap();

    Ok(todo_list)
  }

  async fn get_paginated(&self, page: i64, limit: i64) -> Result<Vec<TodoEntity>, Box<dyn Error>> {
    let todo_list = sqlx::query(
      "
        SELECT * FROM todo ORDER BY created_at DESC LIMIT $1 OFFSET $2
      ",
    )
    .bind(limit)
    .bind(page * limit)
    .map(|i: PgRow| TodoSqlxMapper::pg_row_to_entity(&i))
    .fetch_all(&self.pool)
    .await
    .unwrap();

    Ok(todo_list)
  }
}
