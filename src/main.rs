mod api;
mod api_response;
//mod models;

use crate::api::user_routes;
use crate::api_response::ApiResponse;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
