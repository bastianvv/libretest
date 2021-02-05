use crate::test_plans::{
    CreateSimpleTestPlan,
    UpdateTestPlan,
    TestPlan,
};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testplans")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let test_plans = TestPlan::find_all()?;
    Ok(HttpResponse::Ok().json(test_plans))
}

#[get("/testplans/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let test_plans = TestPlan::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(test_plans))
}

#[post("/testplans")]
async fn create(test_plan: web::Json<CreateSimpleTestPlan>) -> Result<HttpResponse, CustomError> {
    let test_plan = TestPlan::create(test_plan.into_inner())?;
    Ok(HttpResponse::Ok().json(test_plan))
}

#[put("/testplans/{id}")]
async fn update(id: web::Path<i32>, test_plan: web::Json<UpdateTestPlan>) -> Result<HttpResponse, CustomError> {
    let test_plan = TestPlan::update(id.into_inner(), test_plan.into_inner())?;
    Ok(HttpResponse::Ok().json(test_plan))
}

#[delete("/testplans/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_test_plan = TestPlan::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_plan})))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(find_all);
    config.service(create);
    config.service(update);
    config.service(delete);
}