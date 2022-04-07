# base image
FROM rust:1.59.0  
RUN apt update && apt install -y libpq-dev
RUN cargo install diesel_cli -- diesel_cli --no-default-features --features postgres
WORKDIR /blog-service
COPY . .

# RUN Diesel Setup 
RUN cargo install --path 

CMD cargo run 
