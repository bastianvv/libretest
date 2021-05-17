use crate::requirements::{
    CreateSimpleRequirement,
    UpdateRequirement,
    Requirement,
    LinkedRequirement,
    LinkedRequirements
};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use actix_session::Session;
use crate::error_handler::CustomError;

//Requirements
#[get("/requirements")]
async fn find_all(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let requirements = Requirement::find_all()?;
        Ok(HttpResponse::Ok().json(requirements))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/requirements/{id}")]
async fn find(req_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let requirement = Requirement::find(req_id.into_inner())?;
        Ok(HttpResponse::Ok().json(requirement))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/requirements")]
async fn create(requirement: web::Json<CreateSimpleRequirement>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let requirement = Requirement::create(requirement.into_inner())?;
        Ok(HttpResponse::Ok().json(requirement))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/requirements/{id}")]
async fn update(req_id: web::Path<i32>, requirement: web::Json<UpdateRequirement>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let requirement = Requirement::update(req_id.into_inner(), requirement.into_inner())?;
        Ok(HttpResponse::Ok().json(requirement))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/requirements/{id}")]
async fn delete(req_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_requirement = Requirement::delete(req_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_requirement})))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}


//Link requirements
#[get("/requirements/link/{id}")]
async fn find_link(link_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let linked = LinkedRequirements::find(link_id.into_inner())?;
        Ok(HttpResponse::Ok().json(linked))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/requirements/link")]
async fn link(link: web::Json<LinkedRequirement>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let link = LinkedRequirements::link(link.into_inner())?;
        Ok(HttpResponse::Ok().json(link))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/requirements/link/{id}")]
async fn unlink(link_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_link = LinkedRequirements::unlink(link_id.into_inner())?;
        Ok(HttpResponse::Ok().json(deleted_link))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
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
