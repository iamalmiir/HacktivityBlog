use crate::{
    actors::user::find_user_by_email,
    models::user_model::LoginRequest,
    utils::helpers::{error_response, DbPool},
};
use actix_web::{post, web, HttpResponse, Responder, Result};
use bcrypt::verify;
use validator::Validate;

#[post("/auth/login")]
async fn login(pool: web::Data<DbPool>, form: web::Json<LoginRequest>) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let login_req = form.into_inner();
    match login_req.validate() {
        Ok(_) => {
            let user_exists = find_user_by_email(&mut conn, &login_req.email);
            match user_exists {
                Ok(user) => {
                    let passowrd_matches = verify(&login_req.password, &user.password).unwrap();
                    match passowrd_matches {
                        true => Ok(HttpResponse::Ok().json(user)),
                        false => Ok(error_response("Invalid password")),
                    }
                }
                Err(_) => Ok(error_response("Invalid login credentials")),
            }
        }
        Err(_) => Ok(error_response("Data is not valid")),
    }
}
