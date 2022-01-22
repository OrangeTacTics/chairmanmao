mod store;
mod schema;
mod events;

use std::{convert::Infallible};

use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Method, Response, StatusCode,
};

use crate::schema::create_schema;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    let addr = ([0, 0, 0, 0], 8000).into();

    let context = std::sync::Arc::new(tokio::sync::RwLock::new(schema::Context::new().await));
    let schema = std::sync::Arc::new(create_schema());

    let new_service = make_service_fn(move |_| {
        let context = context.clone();
        let schema = schema.clone();

        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let context = context.clone();
                let schema = schema.clone();
                async {
                    let r = match (req.method(), req.uri().path()) {
                        (&Method::GET, "/graphql") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::POST, "/graphql") => {
                            juniper_hyper::graphql(schema, context, req).await
                        }
                        _ => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            response
                        }
                    };
                    Ok::<_, Infallible>(r)
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(new_service);
    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e)
    }
}
