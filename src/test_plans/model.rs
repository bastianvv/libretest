use crate::db;
use crate::error_handler::CustomError;
use crate::schema::test_plans;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime, NaiveDate};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_plans"]
pub struct CreateTestPlan {
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub status: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_plans"]
pub struct CreateSimpleTestPlan {
    pub created_by: i32,
    pub title: String,
}

impl CreateTestPlan {
    fn from(create_test_plan: CreateSimpleTestPlan) -> CreateTestPlan {
        CreateTestPlan {
            creation_date: Utc::now().naive_utc(),
            created_by: create_test_plan.created_by,
            status: 1,
            title: create_test_plan.title,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_plans"]
pub struct UpdateTestPlan {
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "test_plans"]
pub struct TestPlan {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub title: String,
    pub description: Option<String>,
}

impl TestPlan {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let test_plans = test_plans::table.load::<TestPlan>(&conn)?;
        Ok(test_plans)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_plans = test_plans::table.filter(test_plans::id.eq(id)).first(&conn)?;
        Ok(test_plans)
    }

    pub fn create(test_plan: CreateSimpleTestPlan) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_plan = CreateTestPlan::from(test_plan);
        let test_plan = diesel::insert_into(test_plans::table)
            .values(test_plan)
            .get_result(&conn)?;
        Ok(test_plan)
    }

    pub fn update(id: i32, test_plan: UpdateTestPlan) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_plan = diesel::update(test_plans::table)
            .filter(test_plans::id.eq(id))
            .set(test_plan)
            .get_result(&conn)?;
        Ok(test_plan)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(test_plans::table.filter(test_plans::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}