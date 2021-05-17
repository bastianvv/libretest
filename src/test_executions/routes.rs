use crate::test_executions::{
    CreateSimpleTestExecution,
    UpdateTestExecution,
    TestExecution,
};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testexecutions")]
async fn find_all(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_executions = TestExecution::find_all()?;
        Ok(HttpResponse::Ok().json(test_executions))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/testexecutions/{id}")]
async fn find(ex_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_executions = TestExecution::find(ex_id.into_inner())?;
        Ok(HttpResponse::Ok().json(test_executions))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/testexecutions")]
async fn create(test_execution: web::Json<CreateSimpleTestExecution>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_execution = TestExecution::create(test_execution.into_inner())?;
        Ok(HttpResponse::Ok().json(test_execution))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/testexecutions/{id}")]
async fn update(ex_id: web::Path<i32>, test_execution: web::Json<UpdateTestExecution>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_execution = TestExecution::update(ex_id.into_inner(), test_execution.into_inner())?;
        Ok(HttpResponse::Ok().json(test_execution))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/testexecutions/{id}")]
async fn delete(ex_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_test_execution = TestExecution::delete(ex_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_execution})))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
    config.service(update);
    config.service(delete);
}