use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub ssn: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub ssn: Option<String>,
}

