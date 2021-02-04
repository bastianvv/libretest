use crate::db;
use crate::error_handler::CustomError;
use crate::schema::requirements;
use crate::schema::linked_requirements;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Utc, NaiveDateTime};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "requirements"]
pub struct CreateRequirement {
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub status: i32,
    pub title: String,
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "requirements"]
pub struct CreateSimpleRequirement {
    pub created_by: i32,
    pub title: String,
}

impl CreateRequirement {
    fn from(create_requirement: CreateSimpleRequirement) -> CreateRequirement {
        CreateRequirement {
            creation_date: Utc::now().naive_utc(),
            created_by: create_requirement.created_by,
            status: 1,
            title: create_requirement.title,
        }
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Queryable, Insertable)]
#[table_name = "requirements"]
pub struct UpdateRequirement {
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "requirements"]
pub struct Requirement {
    pub id: i32,
    pub creation_date: NaiveDateTime,
    pub created_by: i32,
    pub updated_date: Option<NaiveDateTime>,
    pub updated_by: Option<i32>,
    pub code: Option<String>,
    pub status: i32,
    pub importance: Option<i32>,
    pub nature: Option<i32>,
    pub title: String,
    pub description: Option<String>,
}

impl Requirement {
    //CRUD
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let requirements = requirements::table.load::<Requirement>(&conn)?;
        Ok(requirements)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let requirement = requirements::table.filter(requirements::id.eq(id)).first(&conn)?;
        Ok(requirement)
    }

    pub fn create(requirement: CreateSimpleRequirement) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let requirement = CreateRequirement::from(requirement);
        let requirement = diesel::insert_into(requirements::table)
            .values(requirement)
            .get_result(&conn)?;
        Ok(requirement)
    }

    pub fn update(id: i32, requirement: UpdateRequirement) -> Result<Self, CustomError> {
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

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "linked_requirements"]
pub struct LinkedRequirement {
    parent_id: i32,
    child_id: i32,
}

impl LinkedRequirement {
    fn from(linked_requirements: LinkedRequirement) -> LinkedRequirement {
        LinkedRequirement {
            parent_id: linked_requirements.parent_id,
            child_id: linked_requirements.child_id,
        }
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "linked_requirements"]
pub struct LinkedRequirements {
    id: i32,
    parent_id: i32,
    child_id: i32,
}

impl LinkedRequirements {

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let linked_requirement = linked_requirements::table.filter(linked_requirements::id.eq(id)).first(&conn)?;
        Ok(linked_requirement)
    }

    pub fn link(linked: LinkedRequirement) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let linked = LinkedRequirement::from(linked);
        let linked = diesel::insert_into(linked_requirements::table)
            .values(linked)
            .get_result(&conn)?;
        Ok(linked)
    }

    pub fn unlink(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(linked_requirements::table.filter(linked_requirements::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}