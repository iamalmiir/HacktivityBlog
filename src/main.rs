use actix_session::config::PersistentSession;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::time, web, App, HttpServer};

mod actors;
mod models;
mod response;
mod schema;
mod services;
mod utils;
use services::{auth::login, user::create_user};
use utils::config::{initialize_db_pool, load_config};
use utils::helpers::get_secret_key;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_values = load_config("settings.toml").expect("Failed to load config");
    let secret_val = config_values["server"]["key_secret"].as_str().unwrap();
    let secret_key = get_secret_key(secret_val);
    let pool = initialize_db_pool(config_values["db"]["pgdb"].as_str().unwrap());
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(config_values["db"]["rdb"].as_str().unwrap()),
                    secret_key.clone(),
                )
                .session_lifecycle(
                    PersistentSession::default().session_ttl(time::Duration::days(14)),
                )
                .build(),
            )
            .service(create_user)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
