pub mod delete_todo_case {
  use std::{
    error::Error,
    sync::{Arc, Mutex},
  };

  use async_trait::async_trait;
  use kti_cqrs_rs::common::handler::CommandHandler;

  use crate::contexts::AppContext;

  #[derive(Clone)]
  pub struct Command {
    id: String,
  }

  impl Command {
    pub fn new(id: &str) -> Self {
      Self { id: id.to_string() }
    }
  }

  #[async_trait]
  impl CommandHandler for Command {
    type Context = AppContext;
    type Output = Result<(), Box<dyn Error>>;

    async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
      let repository = context.lock().unwrap().get_command().get_repository();

      repository.delete(&self.id).await
    }
  }
}
