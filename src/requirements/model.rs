use crate::db;
use crate::error_handler::CustomError;
use crate::schema::requirements;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "requirements"]
pub struct Requirement {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub title: String,
    pub description: Option<String>,
}

impl Requirement {
    fn from(requirement: Requirement) -> Requirement {
        Requirement {
            id: requirement.id,
            creation_date: Utc::now().naive_utc(),
            code: requirement.code,
            status: requirement.status,
            importance: requirement.importance,
            nature: requirement.nature,
            title: requirement.title,
            description: requirement.description
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "requirements"]
pub struct Requirements {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub title: String,
    pub description: Option<String>,
}

impl Requirements {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let requirements = requirements::table.load::<Requirements>(&conn)?;
        Ok(requirements)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let requirement = requirements::table.filter(requirements::id.eq(id)).first(&conn)?;
        Ok(requirement)
    }

    pub fn create(requirement: Requirement) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let requirement = Requirement::from(requirement);
        let requirement = diesel::insert_into(requirements::table)
            .values(requirement)
            .get_result(&conn)?;
        Ok(requirement)
    }

    pub fn update(id: i32, requirement: Requirement) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let requirement = diesel::update(requirements::table)
            .filter(requirements::id.eq(id))
            .set(requirement)
            .get_result(&conn)?;
        Ok(requirement)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(requirements::table.filter(requirements::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
