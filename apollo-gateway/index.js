const { ApolloServer } = require('apollo-server');
const { ApolloGateway, RemoteGraphQLDataSource} = require('@apollo/gateway');

const apollo_gateway = new ApolloGateway();
const apollo_server = new ApolloServer( { gateway });

apollo_server.listen().then(({ url }) => { 
    console.log('Hello Testing of Server at ${url}');
}).catch(err => {console.error(err)})