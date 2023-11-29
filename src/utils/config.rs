use actix_web::error::ErrorInternalServerError;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
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

pub fn establish_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to connect to database")
}

pub fn get_database_connection(
    pool: &DbPool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
    pool.get().map_err(|err| {
        eprintln!("Failed to get database connection: {}", err);
        ErrorInternalServerError("Internal Server Error")
    })
}
