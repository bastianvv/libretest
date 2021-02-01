use crate::requirements::{Requirement, Requirements};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

#[get("/requirements")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let requirements = Requirements::find_all()?;
    Ok(HttpResponse::Ok().json(requirements))
}

#[get("/requirements/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirements::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[post("/requirements")]
async fn create(requirement: web::Json<Requirement>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirements::create(requirement.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[put("/requirements/{id}")]
async fn update(id: web::Path<i32>, requirement: web::Json<Requirement>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirements::update(id.into_inner(), requirement.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[delete("/requirements/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_requirement = Requirements::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_requirement})))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
