### Example todo microservice in kti ecosystem.

## Description: this microservice present small implementation of full service with little bit violations of some patterns but its not so important for current example. 

## Patterns implemented are
* Hexagonal architecture
* Domain driven design
* Command Query Responsibility Segregation
* SOLID

## Requirements
* Rust v1.7
* Cargo v1.7
* Linux/Mac
* Postgres
* Proto compiler (protoc | protobuff)
* Grpc cli
* Sqlx cli

## Installations for linux
* Install rust & cargo, go to https://rustup.rs/
* Postgres - just docker or another solution with root access
* `apt install protobuf-compiler`
* `apt install grpc` or via git https://github.com/grpc/grpc/blob/master/doc/command_line_tool.md
* `cargo install sqlx-cli`

## Installations for mac
* Install rust & cargo, go to https://rustup.rs/
* Postgres - just docker or another solution with root access
* `brew install protoc`
* `brew install grpc_cli`
* `cargo install sqlx-cli`

## Run sqlx migrations
* `cd packages/repository`
* `cp .env.example .env`
* `sqlx migrate run`

## Bootstrap
* `cargo build --workspace`
* `cargo run -p todo_api`

## Usage grpc by cli
* `grpc_cli ls localhost:50051` -- test the connection
* `grpc_cli ls localhost:50051 todo_package.TodoService -l` -- show the proto service
* `grpc_cli call localhost:50051 GetPaginated "page: 0, limit: 10"` -- get paginated todo
* `grpc_cli call localhost:50051 Create "name: 'Read the book', description: 'I would like to read 10 pages'"` -- create todo
* `grpc_cli call localhost:50051 Update "id: '112e6968-3424-4575-8bde-a16bcf64eeb6', completed: true"` -- update todo
* `grpc_cli call localhost:50051 GetById "id: '2ad31b72-d18d-4aaf-89fc-9423755fa878'"` -- get by id
* `grpc_cli call localhost:50051 GetByCompleted "completed: true"` -- get paginated by completed
* `grpc_cli call localhost:50051 Delete "id: '2ad31b72-d18d-4aaf-89fc-9423755fa878'"` -- delete by id
