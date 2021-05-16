use crate::db;
use crate::error_handler::CustomError;
use crate::schema::test_executions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime};
use argon2::Config;
use rand::Rng;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct CreateUser {
    pub username: String,
    pub name: Option<String>,
    pub email: String,
    pub admin: Option<bool>,
    pub user_group_id: Option<i32>,
    pub password: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>
}

impl CreateUser {
    fn from(create_user: CreateUser) -> CreateUser {
        CreateUser {
            username: create_user.username,
            name: create_user.name,
            email: create_user.email,
            admin: create_user.admin,
            user_group_id: create_user.user_group_id,
            password: create_user.password,
            creation_date: Utc::now().naive_utc(),
            updated_date: Utc::now().naive_utc()
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: String,
    pub name: Option<String>,
    pub email: String,
    pub admin: Option<bool>,
    pub user_group_id: Option<i32>,
    pub password: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: Option<String>,
    pub email: String,
    pub admin: Option<bool>,
    pub user_group_id: Option<i32>,
    #[serde(skip_serializing)]
    pub password: String,
    pub creation_date: NaiveDateTime,
    pub updated_date: Option<NaiveDateTime>
}

impl User {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let users = users::table.load::<TestExecution>(&conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let users = users::table.filter(users::id.eq(id)).first(&conn)?;
        Ok(users)
    }

    pub fn create(user: CreateUser) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let mut user = User::from(user);
        user.hash_password()?;
        //let user = CreateUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }

    pub fn update(id: i32, test_execution: UpdateTestExecution) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_execution = diesel::update(test_executions::table)
            .filter(test_executions::id.eq(id))
            .set(test_execution)
            .get_result(&conn)?;
        Ok(test_execution)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(test_executions::table.filter(test_executions::id.eq(id))).execute(&conn)?;
        Ok(res)
    }

    pub fn hash_password(&mut self) -> Result<(), CustomError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();
        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .map_err(|e| CustomError::new(500, format!("Failed to hash password: {}", e)))?;

        Ok(())
    }

    pub fn verify_password(&self, password: &[u8]) -> Result<bool, CustomError> {
        argon2::verify_encoded(&self.password, password)
            .map_err(|e| CustomError::new(500, format!("Failed to verify password: {}", e)))

    }
}