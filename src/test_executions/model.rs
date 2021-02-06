use crate::db;
use crate::error_handler::CustomError;
use crate::schema::test_executions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_executions"]
pub struct CreateTestExecution {
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub test_plan_id: i32,
    pub test_case_id: i32,
    pub execution_status: i32,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_executions"]
pub struct CreateSimpleTestExecution {
    pub created_by: i32,
    pub test_plan_id: i32,
    pub test_case_id: i32,
}

impl CreateTestExecution {
    fn from(create_test_execution: CreateSimpleTestExecution) -> CreateTestExecution {
        CreateTestExecution {
            creation_date: Utc::now().naive_utc(),
            created_by: create_test_execution.created_by,
            test_plan_id: create_test_execution.test_plan_id,
            test_case_id: create_test_execution.test_case_id,
            execution_status: 1,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_executions"]
pub struct UpdateTestExecution {
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub execution_status: i32,
    pub execution_date: Option<NaiveDateTime>,
    pub executed_by: Option<i32>,

}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "test_executions"]
pub struct TestExecution {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub test_plan_id: i32,
    pub test_case_id: i32,
    pub execution_status: i32,
    pub execution_date: Option<NaiveDateTime>,
    pub executed_by: Option<i32>,
}

impl TestExecution {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let test_executions = test_executions::table.load::<TestExecution>(&conn)?;
        Ok(test_executions)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_executions = test_executions::table.filter(test_executions::id.eq(id)).first(&conn)?;
        Ok(test_executions)
    }

    pub fn create(test_execution: CreateSimpleTestExecution) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_execution = CreateTestExecution::from(test_execution);
        let test_execution = diesel::insert_into(test_executions::table)
            .values(test_execution)
            .get_result(&conn)?;
        Ok(test_execution)
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
}