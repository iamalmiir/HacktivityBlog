use crate::response::GenericResponse;
use actix_web::{cookie::Key, HttpResponse};
use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_secret_key(value: &str) -> Key {
    Key::derive_from(value.as_bytes())
}

pub fn error_response(message: &str) -> HttpResponse {
    let response = GenericResponse {
        status: "error".to_string(),
        message: message.to_string(),
    };
    HttpResponse::BadRequest().json(response)
}
