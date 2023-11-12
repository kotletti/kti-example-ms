FROM rust:1.73-alpine as building

WORKDIR /home/rust/app

COPY . .

RUN apk add --no-cache musl-dev protoc
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo check --workspace
RUN cargo build --release --bins -p todo_api

### Final stage;

FROM alpine:3.18 as production

COPY --from=building /home/rust/app/target/release/todo_api /home/rust/app/todo_api
COPY --from=building /home/rust/app/packages/api/.env /home/rust/app/.env

WORKDIR /home/rust/app

CMD ["./todo_api"]
