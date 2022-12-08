FROM rust:latest
    MAINTAINER Benjamin Allan-Rahill <benjamin.allanrahill@gmail.com>
ENV DATABASE_URL=postgres://postgres:1CUhBqvLusL6dJF@pando-psql.internal:5432
WORKDIR /usr/src/app
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
RUN diesel migration run
RUN cargo build --release


CMD ["./target/release/server"]
