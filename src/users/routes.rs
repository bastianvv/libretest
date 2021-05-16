use crate::users::{User, CreateUser, UpdateUser};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let users = User::find_all()?;
    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let users = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
async fn create(user: web::Json<CreateUser>) -> Result<HttpResponse, CustomError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(id: web::Path<i32>, user: web::Json<UpdateUser>) -> Result<HttpResponse, CustomError> {
    let user = User::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_user = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
