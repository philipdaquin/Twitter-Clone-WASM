# Build an image with Rust
FROM rust:1.59.0  
# Set the environment Variables 
ENV CARGO_TERM_COLOR always
# Update OS 
RUN apt-get update && \
    apt-get -y upgrade && \
    apt-get -y install libpq-dev
# Create an empty project for caching /auth-service dependencies 
COPY /common/ ./auth-service/common/
RUN USER=root cargo new --bin /auth-service/docker-build
# Set the working directory to /auth-service/
WORKDIR /auth-service/docker-build
# Copy Cargo Toml, Cargo Lock and insert  into ./
COPY /Cargo.lock ./
COPY /auth-service/Cargo.toml ./
# Cache dependencies 
RUN cargo install --path . --locked


# Copy Files from /auth-service and insert into ./
COPY /auth-service ./
RUN cargo install --path . --locked


FROM ubuntu:20.04
RUN apt update && apt install -y libpq-dev

# The Default Command to execute when runnning the container
CMD ["auth-service"]
# Expose the container port on the specific networks ports at runtime
EXPOSE 8081

