use crate::{
    actors::user::{add_user, find_user_by_email},
    models::user_model::CreateUser,
    response::UserResponse,
    utils::helpers::{error_response, DbPool},
};
use actix_web::{post, web, HttpResponse, Responder, Result};
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
            if find_user_by_email(&mut conn, &user.email).is_ok() {
                return Ok(error_response("User with this email already exists"));
            }
            let new_user_data = CreateUser {
                full_name: user.full_name.to_owned(),
                email: user.email.to_owned(),
                password: user.password.to_owned(),
            };
            let user_result = add_user(&mut conn, &new_user_data);
            match user_result {
                Ok(user) => {
                    let json_response = UserResponse {
                        status: "success".to_string(),
                        message: "User created".to_string(),
                        user,
                    };
                    Ok(HttpResponse::Created().json(json_response))
                }
                Err(_) => Ok(HttpResponse::Forbidden().json("Error")),
            }
        }
        Err(_) => Ok(HttpResponse::BadRequest().json("Bad request")),
    }
}
