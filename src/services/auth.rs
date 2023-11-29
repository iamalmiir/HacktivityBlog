use crate::{
    actors::user::find_user_by_email,
    models::user_model::{AuthCredentials, User},
    utils::helpers::DbPool,
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder, Result};
use bcrypt::verify;
use serde_json::json;
use validator::Validate;

impl User {
    fn authenticate(
        conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
        credentials: AuthCredentials,
    ) -> Result<Self, HttpResponse> {
        let user = find_user_by_email(conn, &credentials.email)
            .map_err(|_e| HttpResponse::Unauthorized().json("Unauthorized"))?;

        if verify(&credentials.password, &user.password).unwrap_or(false) {
            Ok(User {
                id: user.id,
                email: user.email,
                full_name: user.full_name,
                password: user.password,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
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
    let mut conn = pool.get().unwrap();
    let login_req = form.into_inner();
    if login_req.validate().is_ok() {
        match User::authenticate(&mut conn, login_req) {
            Ok(user) => {
                let _ = session.insert("user_email", user.email);
                Ok(HttpResponse::Ok().json(json!({ "status": "OK", "message": "User logged"})))
            }
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        }
    } else {
        Ok(HttpResponse::BadRequest().json("Invalid login credentials"))
    }
}
