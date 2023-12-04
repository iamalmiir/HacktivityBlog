use actix_session::Session;
use actix_web::HttpResponse;
use bcrypt::verify;

use crate::models::user_model::{AuthCredentials, User};

pub struct Auth {}

impl Auth {
    pub fn credentials(
        conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
        credentials: &AuthCredentials,
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
        let user_email: Option<String> = session.get("user_email").unwrap_or(None);

        match user_email {
            Some(email) => {
                session.renew();
                Ok(email)
            }
            None => Err(HttpResponse::Unauthorized().json("Unauthorized")),
        }
    }
}
