use serde_derive::Serialize;

use crate::models::user_model::{User, UserDetails};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub message: String,
    pub user: UserDetails,
}

#[derive(Serialize, Debug)]
pub struct UsersResponse {
    pub status: String,
    pub message: String,
    pub users: Vec<User>,
}
