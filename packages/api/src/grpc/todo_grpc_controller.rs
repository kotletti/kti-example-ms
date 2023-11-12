use kti_cqrs_rs::core::bus::ServiceBus;
use todo_core::{
  cases::{
    CreateTodoCase, DeleteTodoCase, GetTodoByCompletedCase, GetTodoByIdCase, GetTodoPaginatedCase,
    UpdateTodoCase,
  },
  contexts::AppContext,
};

use chrono::Local;
use ioc_container_rs::context::{
  container_context::{ContainerContext, ContainerContextProps},
  context::Context,
};
use kti_cqrs_provider_rs::provider::CqrsProvider;
use todo_common::entities::todo_entity::TodoEntity;
use todo_schema_grpc::{
  packages::todo_package::{
    todo_service_server::TodoService, CreateTodoRequest, CreateTodoResponse, DeleteTodoRequest,
    DeleteTodoResponse, GetTodoByCompletedRequest, GetTodoByCompletedResponse, GetTodoByIdRequest,
    GetTodoByIdResponse, GetTodoPaginatedRequest, GetTodoPaginatedResponse, Todo,
    UpdateTodoRequest, UpdateTodoResponse,
  },
  tonic::{async_trait, Request, Response, Status},
};

pub struct TodoGrpcController {
  context: ContainerContext,
}

impl TodoGrpcController {
  pub fn new(props: ContainerContextProps) -> Self {
    Self {
      context: ContainerContext::new(props),
    }
  }

  fn get_bus(&self) -> Box<CqrsProvider::Provider<AppContext>> {
    self.context.resolve_provider(CqrsProvider::TOKEN_PROVIDER)
  }
}

#[async_trait]
impl TodoService for TodoGrpcController {
  async fn create(
    &self,
    request: Request<CreateTodoRequest>,
  ) -> Result<Response<CreateTodoResponse>, Status> {
    let request = request.into_inner();

    let bus = self.get_bus();

    let command = CreateTodoCase::Command::new(&request.name, &request.description);

    let todo_entity = bus.command(Box::new(command)).await.unwrap();

    Ok(Response::new(CreateTodoResponse {
      status_code: 200,
      message: String::from("CREATED"),
      data: Some(Todo {
        id: todo_entity.get_id().to_string(),
        name: todo_entity.get_name().to_string(),
        description: todo_entity.get_description().to_string(),
        completed: todo_entity.get_completed(),
        created_at: todo_entity.get_created_at().to_string(),
        updated_at: todo_entity.get_updated_at().to_string(),
      }),
    }))
  }

  async fn update(
    &self,
    request: Request<UpdateTodoRequest>,
  ) -> Result<Response<UpdateTodoResponse>, Status> {
    let request = request.into_inner();

    let query_bus = self.get_bus();

    let query = GetTodoByIdCase::Query::new(&request.id);

    let todo_entity = match query_bus.query(Box::new(query)).await.unwrap() {
      Some(r) => r,
      None => return Err(Status::not_found("Not found todo by id.")),
    };

    let todo_name: &str = match &request.name {
      Some(r) => r,
      None => todo_entity.get_name(),
    };

    let todo_description: &str = match &request.description {
      Some(r) => r,
      None => todo_entity.get_description(),
    };

    let todo_completed: bool = match &request.completed {
      Some(r) => *r,
      None => todo_entity.get_completed(),
    };

    let todo_entity = TodoEntity::new(
      todo_entity.get_id(),
      todo_name,
      todo_description,
      todo_completed,
      todo_entity.get_created_at(),
      &Local::now().to_string(),
    );

    let command_bus = self.get_bus();

    let command = UpdateTodoCase::Command::new(&todo_entity);

    let updated_todo = command_bus.command(Box::new(command)).await.unwrap();

    Ok(Response::new(UpdateTodoResponse {
      status_code: 200,
      message: String::from("UPDATED"),
      data: Some(Todo {
        id: updated_todo.get_id().to_string(),
        name: updated_todo.get_name().to_string(),
        description: updated_todo.get_description().to_string(),
        completed: updated_todo.get_completed(),
        created_at: updated_todo.get_created_at().to_string(),
        updated_at: updated_todo.get_updated_at().to_string(),
      }),
    }))
  }

  async fn delete(
    &self,
    request: Request<DeleteTodoRequest>,
  ) -> Result<Response<DeleteTodoResponse>, Status> {
    let request = request.into_inner();

    let query_bus = self.get_bus();

    let query = GetTodoByIdCase::Query::new(&request.id);

    match query_bus.query(Box::new(query)).await.unwrap() {
      None => return Err(Status::not_found("Not found todo by id.")),
      _ => (),
    };

    let command_bus = self.get_bus();

    let command = DeleteTodoCase::Command::new(&request.id);

    command_bus.command(Box::new(command)).await.unwrap();

    Ok(Response::new(DeleteTodoResponse {
      status_code: 200,
      message: String::from("DELETED"),
    }))
  }

  async fn get_by_id(
    &self,
    request: Request<GetTodoByIdRequest>,
  ) -> Result<Response<GetTodoByIdResponse>, Status> {
    let request = request.into_inner();

    let bus = self.get_bus();

    let query = GetTodoByIdCase::Query::new(&request.id);

    match bus.query(Box::new(query)).await.unwrap() {
      Some(r) => Ok(Response::new(GetTodoByIdResponse {
        status_code: 200,
        message: String::from("SUCCESS"),
        data: Some(Todo {
          id: r.get_id().to_string(),
          name: r.get_name().to_string(),
          description: r.get_description().to_string(),
          completed: r.get_completed(),
          created_at: r.get_created_at().to_string(),
          updated_at: r.get_updated_at().to_string(),
        }),
      })),
      None => return Err(Status::not_found("Not found todo by id.")),
    }
  }

  async fn get_by_completed(
    &self,
    request: Request<GetTodoByCompletedRequest>,
  ) -> Result<Response<GetTodoByCompletedResponse>, Status> {
    let request = request.into_inner();

    let bus = self.get_bus();

    let query = GetTodoByCompletedCase::Query::new(request.completed);

    let data = bus
      .query(Box::new(query))
      .await
      .unwrap()
      .iter()
      .map(|i| Todo {
        id: i.get_id().to_string(),
        name: i.get_name().to_string(),
        description: i.get_description().to_string(),
        completed: i.get_completed(),
        created_at: i.get_created_at().to_string(),
        updated_at: i.get_updated_at().to_string(),
      })
      .collect::<Vec<Todo>>();

    Ok(Response::new(GetTodoByCompletedResponse {
      status_code: 200,
      message: String::from("SUCCESS"),
      data,
    }))
  }

  async fn get_paginated(
    &self,
    request: Request<GetTodoPaginatedRequest>,
  ) -> Result<Response<GetTodoPaginatedResponse>, Status> {
    let request = request.into_inner();

    let bus = self.get_bus();

    let query = GetTodoPaginatedCase::Query::new(request.page.into(), request.limit.into());

    let data = bus
      .query(Box::new(query))
      .await
      .unwrap()
      .iter()
      .map(|i| Todo {
        id: i.get_id().to_string(),
        name: i.get_name().to_string(),
        description: i.get_description().to_string(),
        completed: i.get_completed(),
        created_at: i.get_created_at().to_string(),
        updated_at: i.get_updated_at().to_string(),
      })
      .collect::<Vec<Todo>>();

    Ok(Response::new(GetTodoPaginatedResponse {
      status_code: 200,
      message: String::from("SUCCESS"),
      data,
    }))
  }
}
