use crate::{
    actors::user::find_user_by_email,
    models::user_model::{AuthCredentials, UserIdentity},
    utils::helpers::{error_response, DbPool},
};
use actix_web::{post, web, HttpResponse, Responder, Result};
use bcrypt::verify;
use validator::Validate;

impl UserIdentity {
    fn authenticate(
        conn: &mut diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<diesel::PgConnection>,
        >,
        credentials: AuthCredentials,
    ) -> Result<Self, HttpResponse> {
        let user = find_user_by_email(conn, &credentials.email)
            .map_err(|_e| HttpResponse::Unauthorized().json("Unauthorized"))?;

        if verify(&credentials.password, &user.password).unwrap_or(false) {
            Ok(UserIdentity {
                id: user.id,
                email: user.email,
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
) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let login_req = form.into_inner();
    if login_req.validate().is_ok() {
        match UserIdentity::authenticate(&mut conn, login_req) {
            Ok(_) => Ok(HttpResponse::Ok().json("Welcome!")),
            Err(_) => Ok(HttpResponse::Unauthorized().json("Unauthorized")),
        }
    } else {
        Ok(error_response("Invalid login credentials"))
    }
}
