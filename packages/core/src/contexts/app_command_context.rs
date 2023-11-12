use std::sync::Arc;

use kti_cqrs_rs::common::context::Context;
use todo_common::repositories::todo_command_repository::TodoCommandRepository;

#[derive(Clone)]
pub struct AppCommandContext {
  repository: Arc<dyn TodoCommandRepository>,
}

impl AppCommandContext {
  pub fn new(repository: Arc<dyn TodoCommandRepository>) -> Self {
    Self { repository }
  }

  pub fn get_repository(&self) -> Arc<dyn TodoCommandRepository> {
    self.repository.clone()
  }
}

impl Context for AppCommandContext {}
