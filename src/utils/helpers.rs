use actix_web::cookie::Key;
use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn get_secret_key(value: &str) -> Key {
    Key::derive_from(value.as_bytes())
}
