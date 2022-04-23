FROM rust:1.59

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev cmake

# create empty project for caching dependencies
RUN USER=root cargo new --bin /blog-service/docker-build
# Get common_utils and insert into docker build 
COPY /common-utils/ ./blog-service/common_utils
# CD into WORKDIR 
WORKDIR /blog-service/docker-build 
# Copy the current Cargo.lock file and insert into docker-build  
COPY /Cargo.lock ./
# Get the Cargo toml inside the blog service and insert into docker-build 
COPY . ./blog-service/Cargo.toml 

# # cache dependencies
# RUN cargo install --path . --locked

# COPY /blog-service/ ./blog-service

# RUN cargo install --path . --locked

# # Multistage 
# FROM debian:buster-slim

# RUN apt-get update && apt-get install -y libpq-dev curl
# COPY --from=0 /usr/local/cargo/bin/blog-service /usr/local/bin/blog-service
CMD ["blog-service"]
EXPOSE 8080

