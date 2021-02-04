use crate::requirements::{
    CreateSimpleRequirement,
    UpdateRequirement,
    Requirement,
    LinkedRequirement,
    LinkedRequirements
};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/requirements")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let requirements = Requirement::find_all()?;
    Ok(HttpResponse::Ok().json(requirements))
}

#[get("/requirements/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirement::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[post("/requirements")]
async fn create(requirement: web::Json<CreateSimpleRequirement>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirement::create(requirement.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[put("/requirements/{id}")]
async fn update(id: web::Path<i32>, requirement: web::Json<UpdateRequirement>) -> Result<HttpResponse, CustomError> {
    let requirement = Requirement::update(id.into_inner(), requirement.into_inner())?;
    Ok(HttpResponse::Ok().json(requirement))
}

#[delete("/requirements/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_requirement = Requirement::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_requirement})))
}

//Link requirements
#[get("/requirements/link/{id}")]
async fn find_link(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let linked = LinkedRequirements::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(linked))
}

#[post("/requirements/link")]
async fn link(link: web::Json<LinkedRequirement>) -> Result<HttpResponse, CustomError> {
    let link = LinkedRequirements::link(link.into_inner())?;
    Ok(HttpResponse::Ok().json(link))
}

#[delete("/requirements/link/{id}")]
async fn unlink(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_link = LinkedRequirements::unlink(id.into_inner())?;
    Ok(HttpResponse::Ok().json(deleted_link))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(find_link);
    config.service(link);
    config.service(unlink);
}
