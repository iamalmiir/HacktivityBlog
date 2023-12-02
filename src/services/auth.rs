use crate::{
    actors::auth::Auth,
    models::user_model::AuthCredentials,
    utils::{config::get_database_connection, helpers::DbPool},
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder, Result};
use serde_json::json;
use validator::Validate;

#[post("/auth/login")]
async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<AuthCredentials>,
    session: Session,
) -> Result<impl Responder> {
    let mut conn = get_database_connection(&pool)?;

    let user_credentials = form.into_inner();
    if user_credentials.validate().is_ok() {
        match Auth::credentials(&mut conn, &user_credentials) {
            Ok(user) => {
                let _ = session.insert("user_email", user);
                Ok(HttpResponse::Ok().json(json!({ "status": "OK", "message": "User logged"})))
            }
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized!")),
        }
    } else {
        Ok(HttpResponse::BadRequest().json("Invalid login credentials"))
    }
}
