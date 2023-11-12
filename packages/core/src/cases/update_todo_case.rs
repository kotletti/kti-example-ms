pub mod update_todo_case {
  use std::{
    error::Error,
    sync::{Arc, Mutex},
  };

  use async_trait::async_trait;
  use kti_cqrs_rs::common::handler::CommandHandler;
  use todo_common::entities::todo_entity::TodoEntity;

  use crate::contexts::AppContext;

  #[derive(Clone)]
  pub struct Command {
    entity: TodoEntity,
  }

  impl Command {
    pub fn new(entity: &TodoEntity) -> Self {
      Self {
        entity: entity.clone(),
      }
    }
  }

  #[async_trait]
  impl CommandHandler for Command {
    type Context = AppContext;
    type Output = Result<TodoEntity, Box<dyn Error>>;

    async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
      let repository = context.lock().unwrap().get_command().get_repository();

      repository.update(&self.entity).await
    }
  }
}
