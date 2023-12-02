use crate::{
    models::user_model::{AuthCredentials, User},
    utils::{config::get_database_connection, helpers::DbPool},
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder, Result};
use bcrypt::verify;
use serde_json::json;
use validator::Validate;

struct Auth {}

impl Auth {
    fn credentials(
        conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
        credentials: AuthCredentials,
    ) -> Result<String, HttpResponse> {
        let user = User::find_user_by_email(conn, &credentials.email)
            .map_err(|_e| HttpResponse::Unauthorized().json("Unauthorized"))?;

        if verify(&credentials.password, &user.password).unwrap_or(false) {
            Ok(user.email)
        } else {
            Err(HttpResponse::Unauthorized().json("Unauthorized"))
        }
    }
}

#[post("/auth/login")]
async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<AuthCredentials>,
    session: Session,
) -> Result<impl Responder> {
    let mut conn = get_database_connection(&pool)?;

    let user_credentials = form.into_inner();
    if user_credentials.validate().is_ok() {
        match Auth::credentials(&mut conn, user_credentials) {
            Ok(user) => {
                let _ = session.insert("user_email", user);
                Ok(HttpResponse::Ok().json(json!({ "status": "OK", "message": "User logged"})))
            }
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        }
    } else {
        Ok(HttpResponse::BadRequest().json("Invalid login credentials"))
    }
}
