use diesel::{r2d2, PgConnection};
use std::fs::File;
use std::io::Read;
use toml::Value;
type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub fn load_config(filename: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str)?;
    let toml_value: Value = toml::de::from_str(&toml_str)?;
    Ok(toml_value)
}

/// Initialize database connection pool based on `DATABASE_URL` environment variable.
///
/// See more: <https://docs.rs/diesel/latest/diesel/r2d2/index.html>.
pub fn initialize_db_pool(conn_url: &str) -> DbPool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(conn_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
