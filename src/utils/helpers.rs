use actix_web::cookie::Key;

pub fn get_secret_key(value: &str) -> Key {
    Key::derive_from(value.as_bytes())
}
