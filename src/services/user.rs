use crate::{
    actors::auth::Auth,
    models::user_model::{CreateUser, User},
    utils::helpers::{error_response, DbPool},
};
use actix_session::Session;
use actix_web::{delete, post, web, HttpResponse, Responder, Result};
use serde_json::json;
use validator::Validate;

/// @api {post} /api/v1/user/create Create a new user
/// @apiName create_user
/// @apiGroup User
/// @apiVersion 1.0.0
/// @apiParam {String} full_name User's name
/// @apiParam {String} email User's email
/// @apiParam {String} password User's password
#[post("/user/create")]
async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<CreateUser>,
) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let user: CreateUser = form.into_inner();
    match user.validate() {
        Ok(_) => {
            if User::find_user_by_email(&mut conn, &user.email).is_ok() {
                return Ok(error_response("User with this email already exists"));
            }
            let user_result = User::add_user(
                &mut conn,
                CreateUser {
                    full_name: user.full_name.to_owned(),
                    email: user.email.to_owned(),
                    password: user.password.to_owned(),
                },
            );
            match user_result {
                Ok(user) => Ok(HttpResponse::Created().json(json!({
                    "status": "success",
                    "message": "User created",
                    "user": user,
                }))),
                Err(_) => Ok(HttpResponse::Forbidden().json("Error")),
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json("Bad request")),
    }
}

#[delete("/user/me")]
async fn delete_user(pool: web::Data<DbPool>, session: Session) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let user_session = Auth::validate_session(&session).unwrap();
    let valid_user = User::find_user_by_email(&mut conn, &user_session).unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User updated",
        "user": valid_user,
    })))
}
