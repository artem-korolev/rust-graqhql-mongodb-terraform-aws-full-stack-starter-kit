use std::{convert::Infallible, net::SocketAddr};

use futures::Future;
use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Method, Response, StatusCode,
};

use crate::api::Schema;

pub async fn create_server(
    schema: Schema,
    db: (),
    addr: SocketAddr,
) -> impl Future<Output = hyper::Result<()>> {
    let schema = std::sync::Arc::new(schema);
    let db = std::sync::Arc::new(db);

    let new_service = make_service_fn(move |_| {
        let schema = schema.clone();
        let ctx = db.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let schema = schema.clone();
                let ctx = ctx.clone();
                async {
                    Ok::<_, Infallible>(match (req.method(), req.uri().path()) {
                        (&Method::GET, "/") => juniper_hyper::playground("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(schema, ctx, req).await
                        }
                        _ => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            response
                        }
                    })
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    server
}
