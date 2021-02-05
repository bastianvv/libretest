use crate::db;
use crate::error_handler::CustomError;
use crate::schema::{
    test_cases,
    linked_test_cases,
    traceability,
    test_case_steps,
    test_case_gherkin
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_cases"]
pub struct CreateTestCase {
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub status: i32,
    pub test_type: i32,
    pub automated: bool,
    pub title: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "test_cases"]
pub struct CreateSimpleTestCase {
    pub created_by: i32,
    pub test_type: i32,
    pub title: String,
}

impl CreateTestCase {
    fn from(create_test_case: CreateSimpleTestCase) -> CreateTestCase {
        CreateTestCase {
            creation_date: Utc::now().naive_utc(),
            created_by: create_test_case.created_by,
            status: 1,
            test_type: create_test_case.test_type,
            automated: false,
            title: create_test_case.title,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "test_cases"]
pub struct UpdateTestCase {
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub automated: bool,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "test_cases"]
pub struct TestCase {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub test_type: i32,
    pub automated: bool,
    pub title: String,
    pub description: Option<String>,
}

impl TestCase {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let test_cases = test_cases::table.load::<TestCase>(&conn)?;
        Ok(test_cases)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_cases = test_cases::table.filter(test_cases::id.eq(id)).first(&conn)?;
        Ok(test_cases)
    }

    pub fn create(test_case: CreateSimpleTestCase) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_case = CreateTestCase::from(test_case);
        let test_case = diesel::insert_into(test_cases::table)
            .values(test_case)
            .get_result(&conn)?;
        Ok(test_case)
    }

    pub fn update(id: i32, test_case: UpdateTestCase) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let test_case = diesel::update(test_cases::table)
            .filter(test_cases::id.eq(id))
            .set(test_case)
            .get_result(&conn)?;
        Ok(test_case)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(test_cases::table.filter(test_cases::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "linked_test_cases"]
pub struct LinkedTestCase {
    parent_id: i32,
    child_id: i32,
}

impl LinkedTestCase {
    fn from(linked_test_case: LinkedTestCase) -> LinkedTestCase {
        LinkedTestCase {
            parent_id: linked_test_case.parent_id,
            child_id: linked_test_case.child_id,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "linked_test_cases"]
pub struct LinkedTestCases {
    id: i32,
    parent_id: i32,
    child_id: i32,
}

impl LinkedTestCases {

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let linked_test_cases = linked_test_cases::table.filter(linked_test_cases::id.eq(id)).first(&conn)?;
        Ok(linked_test_cases)
    }

    pub fn link(linked: LinkedTestCase) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let linked = LinkedTestCase::from(linked);
        let linked = diesel::insert_into(linked_test_cases::table)
            .values(linked)
            .get_result(&conn)?;
        Ok(linked)
    }

    pub fn unlink(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(linked_test_cases::table.filter(linked_test_cases::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "traceability"]
pub struct Traceability {
    req_id: i32,
    tc_id: i32,
}

impl Traceability {
    fn from(traceability: Traceability) -> Traceability {
        Traceability {
            req_id:traceability.req_id,
            tc_id: traceability.tc_id,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "traceability"]
pub struct Trace {
    id: i32,
    req_id: i32,
    tc_id: i32,
}

impl Trace {

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let trace = traceability::table.filter(traceability::id.eq(id)).first(&conn)?;
        Ok(trace)
    }

    pub fn create(trace: Traceability) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let trace = Traceability::from(trace);
        let trace = diesel::insert_into(traceability::table)
            .values(trace)
            .get_result(&conn)?;
        Ok(trace)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(traceability::table.filter(traceability::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
