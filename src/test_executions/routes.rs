use crate::test_executions::{
    CreateSimpleTestExecution,
    UpdateTestExecution,
    TestExecution,
};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testexecutions")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let test_executions = TestExecution::find_all()?;
    Ok(HttpResponse::Ok().json(test_executions))
}

#[get("/testexecutions/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let test_executions = TestExecution::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(test_executions))
}

#[post("/testexecutions")]
async fn create(test_execution: web::Json<CreateSimpleTestExecution>) -> Result<HttpResponse, CustomError> {
    let test_execution = TestExecution::create(test_execution.into_inner())?;
    Ok(HttpResponse::Ok().json(test_execution))
}

#[put("/testexecutions/{id}")]
async fn update(id: web::Path<i32>, test_execution: web::Json<UpdateTestExecution>) -> Result<HttpResponse, CustomError> {
    let test_execution = TestExecution::update(id.into_inner(), test_execution.into_inner())?;
    Ok(HttpResponse::Ok().json(test_execution))
}

#[delete("/testexecutions/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_test_execution = TestExecution::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_execution})))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
    config.service(update);
    config.service(delete);
}