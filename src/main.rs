use std::io;
use std::sync::Arc;

use actix_cors::Cors;

use actix_web::{web, App, Error, HttpResponse, HttpServer, middleware};
use actix_web::web::Data;

#[macro_use]
extern crate juniper;

use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod replay_query;
mod error;
mod output;
mod input;

use crate::replay_query::{create_schema, Schema};

async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
        let res = data.execute(&st, &())
            .await;
        let json = serde_json::to_string(&res)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(Arc::new(create_schema())))
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
