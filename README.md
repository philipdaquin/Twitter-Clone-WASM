# Real Time Blogging Platform on Rust/ WASM 

## Motivation 
To build a scalable, real-time blog platform powered by Apollo Federation, Web assembly, Kafka and Rust.
The project is fully built in Rust from End to End and it implements decoupled services such as User Management, Posts Service and Comments Service. 
Users are fed with new posts and new comments once they become available. To improve the system's reliability, Kafka is used 
to publish new information on the User's feed/

### Architecture
![image 23](https://github.com/philipdaquin/Twitter-Clone-WASM/assets/85416532/8839b2cb-2c9b-4f48-a373-79f213395549)

### Features
- Account Creation and User Authentication 
- Live Blog and Comment Service
- Rate Limiter and Caching 
- Monitoring Service
### Frontend Clients
- NextJs/ReactJs Version of Client [/app]
- WebAssembly/ Yew version of Client [/frontend] 
## TechStack 
- Tailwind
- Rust Programming Language
- Apollo Federation/ Apollo Server  
- Graphql Client
- Async-GraphQL
- Apache Kafka
- Actix-Web
- Redis
- Diesel 
- Postgresql
- Docker

### Ingestion Service [WIP]
- Scylla/ Cassandra [WIP] && Actix-Web

### Monitoring Tools
- Grafana
- Alert Manager 
- cAdvisor 
- Prometheus 



