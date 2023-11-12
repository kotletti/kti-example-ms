use std::sync::Arc;

use kti_cqrs_rs::common::context::Context;
use todo_common::repositories::todo_query_repository::TodoQueryRepository;

#[derive(Clone)]
pub struct AppQueryContext {
  repository: Arc<dyn TodoQueryRepository>,
}

impl AppQueryContext {
  pub fn new(repository: Arc<dyn TodoQueryRepository>) -> Self {
    Self { repository }
  }

  pub fn get_repository(&self) -> Arc<dyn TodoQueryRepository> {
    self.repository.clone()
  }
}

impl Context for AppQueryContext {}
