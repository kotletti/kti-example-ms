use std::sync::{Arc, Mutex};

use ioc_container_rs::{container::Container, context::container_context::ContainerContextProps};
use kti_cqrs_provider_rs::provider::CqrsProvider;
use todo_config::databases::DatabaseConfig;
use todo_core::contexts::{AppCommandContext, AppContext, AppQueryContext};
use todo_repository::{
  common::database_module::database_module,
  todo::{
    sqlx_command_repository::SqlxCommandRepository, sqlx_query_repository::SqlxQueryRepository,
  },
};
use todo_schema_grpc::{
  packages::todo_package::{todo_service_server::TodoServiceServer, FILE_DESCRIPTOR_SET},
  tonic::transport::{Error, Server},
  tonic_reflection,
};

use crate::grpc::TodoGrpcController;

pub struct GrpcServer {
  address: String,
}

impl GrpcServer {
  pub fn new(address: &str) -> Self {
    GrpcServer {
      address: String::from(address),
    }
  }

  pub async fn run(&self) -> Result<(), Error> {
    let database_config = DatabaseConfig::new();

    let connection =
      database_module::create_postgres_pool(database_config.get_database_url()).await;

    let command_repository = SqlxCommandRepository::new(connection.clone());

    let query_repository = SqlxQueryRepository::new(connection.clone());

    let app_context = Arc::new(Mutex::new(AppContext::new(
      AppCommandContext::new(Arc::new(command_repository)),
      AppQueryContext::new(Arc::new(query_repository)),
    )));

    let container = Container::new();

    container.register(CqrsProvider::TOKEN_PROVIDER, move || {
      Box::new(CqrsProvider::Provider::new(app_context.clone()))
    });

    let container_context_props = ContainerContextProps {
      container,
      providers: vec![CqrsProvider::TOKEN_PROVIDER],
    };

    let tenant_grpc_controller = TodoGrpcController::new(container_context_props);

    let reflection_service = tonic_reflection::server::Builder::configure()
      .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
      .build()
      .unwrap();

    println!("Todo grpc server started at: grpc://{}", &self.address);

    Server::builder()
      .add_service(reflection_service)
      .add_service(TodoServiceServer::new(tenant_grpc_controller))
      .serve(self.address.parse().unwrap())
      .await
  }
}
