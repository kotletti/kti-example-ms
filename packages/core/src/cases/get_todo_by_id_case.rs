pub mod get_todo_by_id_case {
  use std::{
    error::Error,
    sync::{Arc, Mutex},
  };

  use async_trait::async_trait;
  use kti_cqrs_rs::common::handler::QueryHandler;
  use todo_common::entities::todo_entity::TodoEntity;

  use crate::contexts::AppContext;

  #[derive(Clone)]
  pub struct Query {
    id: String,
  }

  impl Query {
    pub fn new(id: &str) -> Self {
      Self { id: id.to_string() }
    }
  }

  #[async_trait]
  impl QueryHandler for Query {
    type Context = AppContext;
    type Output = Result<Option<TodoEntity>, Box<dyn Error>>;

    async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
      let repository = context.lock().unwrap().get_query().get_repository();

      repository.get_by_id(&self.id).await
    }
  }
}
