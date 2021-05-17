use crate::test_plans::{
    CreateSimpleTestPlan,
    UpdateTestPlan,
    TestPlan,
};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testplans")]
async fn find_all(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_plans = TestPlan::find_all()?;
        Ok(HttpResponse::Ok().json(test_plans))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/testplans/{id}")]
async fn find(tp_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_plans = TestPlan::find(tp_id.into_inner())?;
        Ok(HttpResponse::Ok().json(test_plans))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/testplans")]
async fn create(test_plan: web::Json<CreateSimpleTestPlan>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_plan = TestPlan::create(test_plan.into_inner())?;
        Ok(HttpResponse::Ok().json(test_plan))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/testplans/{id}")]
async fn update(tp_id: web::Path<i32>, test_plan: web::Json<UpdateTestPlan>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_plan = TestPlan::update(tp_id.into_inner(), test_plan.into_inner())?;
        Ok(HttpResponse::Ok().json(test_plan))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/testplans/{id}")]
async fn delete(tp_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_test_plan = TestPlan::delete(tp_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_plan})))
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