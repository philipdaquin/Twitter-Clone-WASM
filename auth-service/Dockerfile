FROM rust:1.59

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev cmake

# create empty project for caching dependencies
RUN USER=root cargo new --bin /auth-service/docker-build
# Get common_utils and insert into docker build 
COPY /common-utils/ ./auth-service/common_utils
# CD into WORKDIR 
WORKDIR /auth-service/docker-build 
# Copy the current Cargo.lock file and insert into docker-build  
COPY /Cargo.lock ./
# Get the Cargo toml inside the auth service and insert into docker-build 
COPY . ./auth-service/Cargo.toml 

# # cache dependencies
# RUN cargo install --path . --locked

# COPY /auth-service/ ./auth-service

# RUN cargo install --path . --locked

# # Multistage 
# FROM debian:buster-slim

# RUN apt-get update && apt-get install -y libpq-dev curl
# COPY --from=0 /usr/local/cargo/bin/auth-service /usr/local/bin/auth-service
CMD ["auth-service"]
EXPOSE 8080

