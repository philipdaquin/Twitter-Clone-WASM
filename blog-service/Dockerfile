FROM rust:1.50

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev cmake

# create empty project for caching dependencies
RUN USER=root cargo new --bin /blog-service/docker-build
WORKDIR /blog-service/docker-build
COPY /Cargo.lock ./
COPY /blog-service/Cargo.toml ./
COPY /../common_utils/ /blog-service/docker-build/
# cache dependencies
RUN cargo install --path . --locked
COPY /blog-service/ ./
RUN cargo install --path . --locked
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=0 /usr/local/cargo/bin/blog-service /usr/local/bin/blog-service
CMD ["blog-service"]
EXPOSE 8080

