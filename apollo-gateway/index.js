const { ApolloServer } = require('apollo-server');
const { ApolloGateway, RemoteGraphQLDataSource, 
    IntrospectAndCompose
} = require('@apollo/gateway');


class AuthenticatedDataSource extends RemoteGraphQLDataSource { 
    willSendRequest({request, context}) { 
        if (context.authHeaderValue) { 
            request.http.headers.set('Authorization', context.authHeaderValue);
        }
    }
}
let node_env = process.env.NODE_ENV;
function get_service_url(service_name, port) { 
    let host; 
    switch (node_env) { 
        case 'docker': 
            host = service_name;
            break;
        case 'local': { 
            host: 'localhost';
            break
        }
    }
    return "http://" + host + ":" + port;
}

const gateway = new ApolloGateway({
    supergraphSdl: new IntrospectAndCompose({
        subgraphs: [
            {name: "auth-service", url: get_service_url("auth-service", 8081)},
            {name: "blog-service", url: get_service_url("blog-service", 8080)},
        ],
        buildService({name, url}) {
            return new AuthenticatedDataSource({ url });
        },
        
    }),
});

const apollo_server = new ApolloServer( 
    { 
        gateway, 
        engine: false,
        //  Subscriptions are unsupported but planned for a future gateway verison  
        subscriptions: false, 
        context: ({ req}) => ({
            authHeaderValue: req.headers.authorization
        })
    }
);


apollo_server.listen( {host: "0.0.0.0", port: 4000 }).then(({ url }) => { 
    console.log('Hello Testing of Server at ${url} 🚀'); 
    }).catch(err => { console.error(err) })