#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;

const DATABASE_URL : &str = "DATABASE_URL";
const HOST : &str = "HOST";
const PORT : &str = "PORT";
const RUST_LOG : &str = "RUST_LOG";

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
    "#
    )
}


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var(DATABASE_URL).expect("DATABASE_URL is not set in env");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    let mut server = HttpServer::new(move || {
       App::new()
           .data(db_pool.clone())
           .route("/", web::get().to(index))
    });

    let host = env::var(HOST).expect("HOST is not set in env");
    let port = env::var(PORT).expect("PORT is not set in env");

    server
        .bind(format!("{}:{}", host, port))?
        .run().await?;

    Ok(())
}
