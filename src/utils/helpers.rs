use actix_web::{cookie::Key, HttpResponse};

use crate::response::GenericResponse;

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
