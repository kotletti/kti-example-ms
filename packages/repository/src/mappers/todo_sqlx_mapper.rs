use sqlx::{
  postgres::PgRow,
  types::{chrono::NaiveDateTime, Uuid},
  Row,
};
use todo_common::entities::todo_entity::TodoEntity;

pub struct TodoSqlxMapper;

impl TodoSqlxMapper {
  pub fn pg_row_to_entity(pg_row: &PgRow) -> TodoEntity {
    TodoEntity::new(
      &pg_row.try_get::<Uuid, &str>("id").unwrap().to_string(),
      pg_row.try_get("name").unwrap(),
      pg_row.try_get("description").unwrap(),
      pg_row.try_get("completed").unwrap(),
      &pg_row
        .try_get::<NaiveDateTime, &str>("created_at")
        .unwrap()
        .to_string(),
      &pg_row
        .try_get::<NaiveDateTime, &str>("updated_at")
        .unwrap()
        .to_string(),
    )
  }
}
