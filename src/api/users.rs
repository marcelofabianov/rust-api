use crate::services::UserService;
use actix_web::web::{get, scope, Data, ServiceConfig};
use actix_web::{HttpResponse, Responder};
use sqlx::PgPool;

pub async fn list_users(pool: Data<PgPool>) -> impl Responder {
    let user_service = UserService::new(pool.get_ref().clone());

    match user_service.find_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch users"),
    }
}

pub fn user_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").route("/users", get().to(list_users)));
}
