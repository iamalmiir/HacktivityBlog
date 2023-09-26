use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

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
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
}
