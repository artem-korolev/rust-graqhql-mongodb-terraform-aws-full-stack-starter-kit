use std::{convert::Infallible, net::SocketAddr};

use futures::Future;
use hyper::{server::Server, Body, Request, Response};
use routerify::{Router, RouterService};
// Import the CORS crate.
use routerify_cors::enable_cors_all;

use crate::{graphql::Schema, repository::Repository};

pub async fn create_server(
    schema: Schema,
    db: Repository,
    addr: SocketAddr,
) -> impl Future<Output = hyper::Result<()>> {
    // let schema = std::sync::Arc::new(schema);
    // let db = std::sync::Arc::new(db);

    // let new_service = make_service_fn(move |_| {
    //     let schema = schema.clone();
    //     let ctx = db.clone();
    //     async {
    //         Ok::<_, hyper::Error>(service_fn(move |req| {
    //             let schema = schema.clone();
    //             let ctx = ctx.clone();
    //             async {
    //                 let mut resp = match (req.method(), req.uri().path()) {
    //                     (&Method::GET, "/") => juniper_hyper::playground("/graphql", None).await,
    //                     (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
    //                         juniper_hyper::graphql(schema, ctx, req).await
    //                     }
    //                     (&Method::OPTIONS, "/graphql") => {
    //                         let mut response = Response::new(Body::empty());
    //                         *response.status_mut() = StatusCode::OK;
    //                         response
    //                     }
    //                     _ => {
    //                         let mut response = Response::new(Body::empty());
    //                         *response.status_mut() = StatusCode::NOT_FOUND;
    //                         response
    //                     }
    //                 };
    //                 // resp.headers_mut().insert(
    //                 //     hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
    //                 //     HeaderValue::from_str("*").unwrap(),
    //                 // );
    //                 Ok::<_, Infallible>(resp)
    //             }
    //         }))
    //     }
    // });

    let router = {
        let schema = std::sync::Arc::new(schema);
        let db = std::sync::Arc::new(db);

        Router::builder()
            // Attach the CORS middleware.
            .middleware(enable_cors_all())
            .post("/graphql", move |req| {
                graphql_handler(req, schema.clone(), db.clone())
            })
            .get("/", |_req| playground_handler())
            .build()
            .unwrap()
    };

    // Create a Service from the router above to handle incoming requests.
    let service = RouterService::new(router).unwrap();

    Server::bind(&addr).serve(service)
}

async fn graphql_handler(
    req: Request<Body>,
    schema: std::sync::Arc<Schema>,
    repo: std::sync::Arc<Repository>,
) -> Result<Response<Body>, Infallible> {
    Ok(juniper_hyper::graphql(schema, repo, req).await)
}

async fn playground_handler() -> Result<Response<Body>, Infallible> {
    Ok(juniper_hyper::playground("/graphql", None).await)
}
