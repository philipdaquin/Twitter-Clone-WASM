FROM rust:latest 

RUN apt-get update \
    apt-get install -y libpq-dev clang

WORKDIR /auth

COPY . /auth/auth-service

COPY ../common-utils/ ./auth

WORKDIR /auth-service

RUN cargo build --release 

CMD ["auth-service"]

EXPOSE 8081


