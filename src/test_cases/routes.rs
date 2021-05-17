use crate::test_cases::{CreateSimpleTestCase, UpdateTestCase, TestCase, LinkedTestCase, LinkedTestCases, Traceability, Trace, TestCaseStep, TestCaseSteps, TestCaseGherkin, Gherkin, UpdateTestCaseSteps, UpdateTestCaseGherkin};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;

//Requirements
#[get("/testcases")]
async fn find_all(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_cases = TestCase::find_all()?;
        Ok(HttpResponse::Ok().json(test_cases))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/testcases/{id}")]
async fn find(tc_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_case = TestCase::find(tc_id.into_inner())?;
        Ok(HttpResponse::Ok().json(test_case))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/testcases")]
async fn create(test_case: web::Json<CreateSimpleTestCase>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_case = TestCase::create(test_case.into_inner())?;
        Ok(HttpResponse::Ok().json(test_case))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/testcases/{id}")]
async fn update(tc_id: web::Path<i32>, test_case: web::Json<UpdateTestCase>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_case = TestCase::update(tc_id.into_inner(), test_case.into_inner())?;
        Ok(HttpResponse::Ok().json(test_case))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/testcases/{id}")]
async fn delete(tc_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_test_case = TestCase::delete(tc_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_test_case})))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

//Link test cases
#[get("/testcases/link/{id}")]
async fn find_link(link_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let linked = LinkedTestCases::find(link_id.into_inner())?;
        Ok(HttpResponse::Ok().json(linked))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/testcases/link")]
async fn link(link: web::Json<LinkedTestCase>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let link = LinkedTestCases::link(link.into_inner())?;
        Ok(HttpResponse::Ok().json(link))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/testcases/link/{id}")]
async fn unlink(link_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_link = LinkedTestCases::unlink(link_id.into_inner())?;
        Ok(HttpResponse::Ok().json(deleted_link))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

//Traceability
#[get("/traceability/{id}")]
async fn find_traceability(trace_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let trace = Trace::find(trace_id.into_inner())?;
        Ok(HttpResponse::Ok().json(trace))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/traceability")]
async fn traceability(trace: web::Json<Traceability>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let trace = Trace::create(trace.into_inner())?;
        Ok(HttpResponse::Ok().json(trace))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/traceability/{id}")]
async fn delete_traceability(trace_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_traceability = Trace::delete(trace_id.into_inner())?;
        Ok(HttpResponse::Ok().json(deleted_traceability))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

//Test Case Steps
#[get("/steps/{id}")]
async fn find_step(step_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let step = TestCaseStep::find(step_id.into_inner())?;
        Ok(HttpResponse::Ok().json(step))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/steps")]
async fn create_step(step: web::Json<TestCaseSteps>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let step = TestCaseStep::create(step.into_inner())?;
        Ok(HttpResponse::Ok().json(step))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/steps/{id}")]
async fn update_step(step_id: web::Path<i32>, test_case_steps: web::Json<UpdateTestCaseSteps>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let test_case_steps = TestCaseStep::update(step_id.into_inner(), test_case_steps.into_inner())?;
        Ok(HttpResponse::Ok().json(test_case_steps))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/steps/{id}")]
async fn delete_step(step_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_step = TestCaseStep::delete(step_id.into_inner())?;
        Ok(HttpResponse::Ok().json(deleted_step))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

//Test Case Steps
#[get("/gherkin/{id}")]
async fn find_gherkin(gherkin_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let gherkin = Gherkin::find(gherkin_id.into_inner())?;
        Ok(HttpResponse::Ok().json(gherkin))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/gherkin")]
async fn create_gherkin(gherkin: web::Json<TestCaseGherkin>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let gherkin = Gherkin::create(gherkin.into_inner())?;
        Ok(HttpResponse::Ok().json(gherkin))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[put("/gherkin/{id}")]
async fn update_gherkin(gherkin_id: web::Path<i32>, gherkin: web::Json<UpdateTestCaseGherkin>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let gherkin = Gherkin::update(gherkin_id.into_inner(), gherkin.into_inner())?;
        Ok(HttpResponse::Ok().json(gherkin))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/gherkin/{id}")]
async fn delete_gherkin(gherkin_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_gherkin = Gherkin::delete(gherkin_id.into_inner())?;
        Ok(HttpResponse::Ok().json(deleted_gherkin))
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
