// src/main.rs
mod db;
mod handlers;
mod models;
mod schema;
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load environment variables from .env file

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);

    println!("Starting server at http://{}", server_addr);

    HttpServer::new(|| {
        App::new()
            .route("/users", web::get().to(handlers::get_users))
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/{id}", web::put().to(handlers::update_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&server_addr)?
    .run()
    .await
}
