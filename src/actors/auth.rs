use actix_web::HttpResponse;
use bcrypt::verify;

use crate::models::user_model::{AuthCredentials, User};
use actix_session::Session;

pub struct Auth {}

impl Auth {
    pub fn credentials(
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

    pub fn validate_session(session: &Session) -> Result<String, HttpResponse> {
        let user_email = session.get::<String>("user_email").unwrap_or_default();

        if user_email.is_none() {
            Err(HttpResponse::Unauthorized().json("Unauthorized"))
        } else {
            Ok(user_email.unwrap())
        }
    }
}
