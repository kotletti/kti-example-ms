use kti_cqrs_rs::common::context::Context;

use super::{AppCommandContext, AppQueryContext};

pub struct AppContext {
  command: AppCommandContext,
  query: AppQueryContext,
}

impl AppContext {
  pub fn new(command: AppCommandContext, query: AppQueryContext) -> Self {
    Self { command, query }
  }

  pub fn get_command(&self) -> AppCommandContext {
    self.command.clone()
  }

  pub fn get_query(&self) -> AppQueryContext {
    self.query.clone()
  }
}

impl Context for AppContext {}
