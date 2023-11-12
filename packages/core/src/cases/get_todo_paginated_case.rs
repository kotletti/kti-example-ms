pub mod get_todo_paginated_case {
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
    page: i64,
    limit: i64,
  }

  impl Query {
    pub fn new(page: i64, limit: i64) -> Self {
      Self { page, limit }
    }
  }

  #[async_trait]
  impl QueryHandler for Query {
    type Context = AppContext;
    type Output = Result<Vec<TodoEntity>, Box<dyn Error>>;

    async fn execute(&self, context: Arc<Mutex<Self::Context>>) -> Self::Output {
      let repository = context.lock().unwrap().get_query().get_repository();

      repository.get_paginated(self.page, self.limit).await
    }
  }
}
