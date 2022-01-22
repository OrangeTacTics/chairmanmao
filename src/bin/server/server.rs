use std::{convert::Infallible};

use hyper::{
    server::Server,
    service::{make_service_fn, service_fn},
    Body, Method, Response, StatusCode,
};

mod schema;
use crate::schema::{create_schema, Schema};

struct State {
    schema: std::sync::Arc<Schema>,
    context: schema::Context,
}

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
                        (&Method::GET, "/") => juniper_hyper::graphiql("/graphql", None).await,
                        (&Method::GET, "/graphql") | (&Method::POST, "/graphql") => {
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

/*
//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, guard};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod schema;
use crate::schema::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[derive(Clone)]
struct State {
    schema: std::sync::Arc<Schema>,
    context: schema::Context,
}

async fn graphql(
    state: web::Data<State>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let result = data.execute(&state.schema, &state.context).await;
    let json = serde_json::to_string(&result)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create Juniper schema
    let schema = std::sync::Arc::new(create_schema());
    let context = schema::Context::new().await;

    let state = State {
        schema,
        context,
    };

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/graphql").guard(guard::Get()).to(graphiql))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
*/
