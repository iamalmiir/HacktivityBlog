use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
// use validator_derive::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDetails {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1))]
    pub full_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize, Debug, Serialize, Clone, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}
