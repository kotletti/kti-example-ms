#[derive(Clone)]
pub struct TodoEntity {
  id: String,
  name: String,
  description: String,
  completed: bool,
  created_at: String,
  updated_at: String,
}

impl TodoEntity {
  pub fn new(
    id: &str,
    name: &str,
    description: &str,
    completed: bool,
    created_at: &str,
    updated_at: &str,
  ) -> TodoEntity {
    TodoEntity {
      id: id.to_string(),
      name: name.to_string(),
      description: description.to_string(),
      completed,
      created_at: created_at.to_string(),
      updated_at: updated_at.to_string(),
    }
  }

  pub fn get_id(&self) -> &str {
    &self.id
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_description(&self) -> &str {
    &self.description
  }

  pub fn get_completed(&self) -> bool {
    self.completed
  }

  pub fn get_created_at(&self) -> &str {
    &self.created_at
  }

  pub fn get_updated_at(&self) -> &str {
    &self.updated_at
  }
}
