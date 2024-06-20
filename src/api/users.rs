use crate::api_response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").route("/users", web::get().to(list_users)));
}

pub async fn list_users() -> impl Responder {
    let data = vec!["Alice".to_string(), "Bob".to_string()];
    let response = ApiResponse::success(data);

    HttpResponse::Ok().json(response)
}
