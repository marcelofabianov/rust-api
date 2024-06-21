mod api;
mod api_response;
mod db;
mod models;
mod services;

use crate::api::user_routes;
use crate::api_response::ApiResponse;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::connect().await.expect("Failed to connect to database");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(default))
            .configure(user_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn default() -> impl Responder {
    let response = ApiResponse::success("Default response".to_string());
    HttpResponse::Ok().json(response)
}
