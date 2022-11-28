mod models;
use std::sync::Arc;

use models::*;
mod database;
use database::*;

mod routes;
use routes::*;

use actix_web::{get, web, App, HttpServer, Responder};
use dashmap::DashMap;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

struct AppData {
    pub tokens: Arc<DashMap<String, AccessToken>>,
    pub users: Arc<DashMap<String, [u8; 4]>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tokens: Arc<DashMap<String, AccessToken>> = Arc::new(DashMap::new());
    HttpServer::new(move || {
        App::new()
            .app_data(tokens.clone())
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
