use crate::actors::user::find_user_by_email;
use actix_web::{post, web, HttpResponse, Responder, Result};
use bcrypt::verify;
use diesel::{r2d2, PgConnection};
use serde_derive::Deserialize;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[derive(Deserialize, Debug)]
struct LoginRequest {
    email: String,
    password: String,
}

#[post("/auth/login")]
async fn login(pool: web::Data<DbPool>, form: web::Json<LoginRequest>) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let user_exists = find_user_by_email(&mut conn, &form.email);

    match user_exists {
        Ok(user) => {
            if verify(&form.password, &user.password).unwrap() {
                Ok(HttpResponse::Ok().json(user))
            } else {
                Err(actix_web::error::ErrorUnauthorized("Invalid password"))
            }
        }
        Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid email")),
    }
}
