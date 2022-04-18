FROM rust:stable

RUN apt-get update && apt-get install -y
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk 

WORKDIR /frontend

COPY ./ ./ 

RUN cargo build --release
RUN trunk build --release

ENTRYPOINT [ "trunk", "serve" ]
EXPOSE 8080