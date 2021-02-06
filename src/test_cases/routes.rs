use crate::test_cases::{CreateSimpleTestCase, UpdateTestCase, TestCase, LinkedTestCase, LinkedTestCases, Traceability, Trace, TestCaseStep, TestCaseSteps, TestCaseGherkin, Gherkin, UpdateTestCaseSteps, UpdateTestCaseGherkin};

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testcases")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let test_cases = TestCase::find_all()?;
    Ok(HttpResponse::Ok().json(test_cases))
}

#[get("/testcases/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let test_case = TestCase::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(test_case))
}

#[post("/testcases")]
async fn create(test_case: web::Json<CreateSimpleTestCase>) -> Result<HttpResponse, CustomError> {
    let test_case = TestCase::create(test_case.into_inner())?;
    Ok(HttpResponse::Ok().json(test_case))
}

#[put("/testcases/{id}")]
async fn update(id: web::Path<i32>, test_case: web::Json<UpdateTestCase>) -> Result<HttpResponse, CustomError> {
    let test_case = TestCase::update(id.into_inner(), test_case.into_inner())?;
    Ok(HttpResponse::Ok().json(test_case))
}

#[delete("/testcases/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_test_case = TestCase::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_case})))
}

//Link test cases
#[get("/testcases/link/{id}")]
async fn find_link(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let linked = LinkedTestCases::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(linked))
}

#[post("/testcases/link")]
async fn link(link: web::Json<LinkedTestCase>) -> Result<HttpResponse, CustomError> {
    let link = LinkedTestCases::link(link.into_inner())?;
    Ok(HttpResponse::Ok().json(link))
}

#[delete("/testcases/link/{id}")]
async fn unlink(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_link = LinkedTestCases::unlink(id.into_inner())?;
    Ok(HttpResponse::Ok().json(deleted_link))
}

//Traceability
#[get("/traceability/{id}")]
async fn find_traceability(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let trace = Trace::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(trace))
}

#[post("/traceability")]
async fn traceability(trace: web::Json<Traceability>) -> Result<HttpResponse, CustomError> {
    let trace = Trace::create(trace.into_inner())?;
    Ok(HttpResponse::Ok().json(trace))
}

#[delete("/traceability/{id}")]
async fn delete_traceability(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_traceability = Trace::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(deleted_traceability))
}

//Test Case Steps
#[get("/steps/{id}")]
async fn find_step(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let step = TestCaseStep::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(step))
}

#[post("/steps")]
async fn create_step(step: web::Json<TestCaseSteps>) -> Result<HttpResponse, CustomError> {
    let step = TestCaseStep::create(step.into_inner())?;
    Ok(HttpResponse::Ok().json(step))
}

#[put("/steps/{id}")]
async fn update_step(id: web::Path<i32>, test_case_steps: web::Json<UpdateTestCaseSteps>) -> Result<HttpResponse, CustomError> {
    let test_case_steps = TestCaseStep::update(id.into_inner(), test_case_steps.into_inner())?;
    Ok(HttpResponse::Ok().json(test_case_steps))
}

#[delete("/steps/{id}")]
async fn delete_step(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_step = TestCaseStep::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(deleted_step))
}

//Test Case Steps
#[get("/gherkin/{id}")]
async fn find_gherkin(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let gherkin = Gherkin::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(gherkin))
}

#[post("/gherkin")]
async fn create_gherkin(gherkin: web::Json<TestCaseGherkin>) -> Result<HttpResponse, CustomError> {
    let gherkin = Gherkin::create(gherkin.into_inner())?;
    Ok(HttpResponse::Ok().json(gherkin))
}

#[put("/gherkin/{id}")]
async fn update_gherkin(id: web::Path<i32>, gherkin: web::Json<UpdateTestCaseGherkin>) -> Result<HttpResponse, CustomError> {
    let gherkin = Gherkin::update(id.into_inner(), gherkin.into_inner())?;
    Ok(HttpResponse::Ok().json(gherkin))
}

#[delete("/gherkin/{id}")]
async fn delete_gherkin(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_gherkin = Gherkin::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(deleted_gherkin))
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
    config.service(find_traceability);
    config.service(traceability);
    config.service(delete_traceability);
    config.service(find_step);
    config.service(create_step);
    config.service(update_step);
    config.service(delete_step);
    config.service(find_gherkin);
    config.service(create_gherkin);
    config.service(update_gherkin);
    config.service(delete_gherkin);
}
