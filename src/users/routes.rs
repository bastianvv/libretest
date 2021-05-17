use crate::users::{User, CreateUser, UpdateUser, AuthUser};
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_session::Session;
use serde_json::json;
use crate::error_handler::CustomError;

#[get("/users")]
async fn find_all(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let users = User::find_all()?;
        Ok(HttpResponse::Ok().json(users))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[get("/users/{id}")]
async fn find(user_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let users = User::find(user_id.into_inner())?;
        Ok(HttpResponse::Ok().json(users))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/users")]
async fn create(user: web::Json<CreateUser>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let user = User::create(user.into_inner())?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }

}

#[put("/users/{id}")]
async fn update(user_id: web::Path<i32>, user: web::Json<UpdateUser>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let user = User::update(user_id.into_inner(), user.into_inner())?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[delete("/users/{id}")]
async fn delete(user_id: web::Path<i32>, session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;
    if let Some(id) = id {
        let deleted_user = User::delete(user_id.into_inner())?;
        Ok(HttpResponse::Ok().json(json!({"deleted": deleted_user})))
    } else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

#[post("/login")]
async fn login(credentials: web::Json<AuthUser>, session: Session) -> Result<HttpResponse, CustomError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_username(credentials.username)
        .map_err(|e| {
            match e.error_status_code {
                404 => CustomError::new(401, "Credentials not valid!".to_string()),
                _ => e,
            }
        })?;

    let is_valid = user.verify_password(credentials.password.as_bytes())?;
    if is_valid == true {
        session.set("user_id", user.id);
        session.renew();
        Ok(HttpResponse::Ok().json(json!({"message": "Login successful"})))
    }
    else {
        Err(CustomError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/logout")]
async fn logout(session: Session) -> Result<HttpResponse, CustomError> {
    let id: Option<i32> = session.get("user_id")?;

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({ "message": "Successfully signed out" })))
    }
    else {
        Err(CustomError::new(401, "Unauthorized".to_string()))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(login);
    cfg.service(logout);
}
