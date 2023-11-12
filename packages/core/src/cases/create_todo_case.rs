pub mod create_todo_case {
  use std::{
    error::Error,
    sync::{Arc, Mutex},
  };

  use async_trait::async_trait;
  use kti_cqrs_rs::common::handler::CommandHandler;
  use sqlx::types::chrono::Local;
  use todo_common::entities::todo_entity::TodoEntity;
  use uuid::Uuid;

  use crate::contexts::AppContext;

  #[derive(Clone)]
  pub struct Command {
    name: String,
    description: String,
  }

  impl Command {
    pub fn new(name: &str, description: &str) -> Self {
      Self {
        name: name.to_string(),
        description: description.to_string(),
      }
    }
  }

  #[async_trait]
  impl CommandHandler for Command {
    type Context = AppContext;
    type Output = Result<TodoEntity, Box<dyn Error>>;

    async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
      let repository = context.lock().unwrap().get_command().get_repository();

      repository
        .create(&TodoEntity::new(
          &Uuid::new_v4().to_string(),
          &self.name,
          &self.description,
          false,
          &Local::now().to_string(),
          &Local::now().to_string(),
        ))
        .await
    }
  }
}
