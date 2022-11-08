use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, 
    GraphQLEnum, Variables, graphql_value,
};

#[derive(GraphQLEnum, Clone, Copy)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

// Arbitrary context data.
struct Ctx(Episode);

impl juniper::Context for Ctx {}

struct Query;

#[graphql_object(context = Ctx)]
impl Query {
    fn favoriteEpisode(context: &Ctx) -> FieldResult<Episode> {
        Ok(context.0)
    }
}

// A root schema consists of a query, a mutation, and a subscription.
// Request queries can be executed against a RootNode.
type Schema = juniper::RootNode<'static, Query, EmptyMutation<Ctx>, EmptySubscription<Ctx>>;







use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};


async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

// fn main() {
//     // Create a context object.
//     let ctx = Ctx(Episode::NewHope);

//     // Run the executor.
//     let (res, _errors) = juniper::execute_sync(
//         "query { favoriteEpisode }",
//         None,
//         &Schema::new(Query, EmptyMutation::new(), EmptySubscription::new()),
//         &Variables::new(),
//         &ctx,
//     ).unwrap();

//     // Ensure the value matches.
//     assert_eq!(
//         res,
//         graphql_value!({
//             "favoriteEpisode": "NEW_HOPE",
//         })
//     );
// }