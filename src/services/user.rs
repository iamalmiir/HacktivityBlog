use crate::models;
use crate::response::{GenericResponse, UserResponse};
use crate::{
    actors::user::{add_user, find_user_by_email},
    models::user_model::NewUser,
};
use actix_web::{post, web, HttpResponse, Responder, Result};
use diesel::{r2d2, PgConnection};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

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
    form: web::Json<models::user_model::NewUser>,
) -> Result<impl Responder> {
    let mut conn = pool.get().unwrap();
    let new_user: NewUser = form.into_inner();
    // Check if the user already exists in the database
    if find_user_by_email(&mut conn, &new_user.email).is_ok() {
        let error_response = GenericResponse {
            status: "error".to_string(),
            message: "User with this email already exists".to_string(),
        };

        return Ok(HttpResponse::BadRequest().json(error_response));
    }
    // Create the user in the database
    let user_result = add_user(&mut conn, &new_user);
    match user_result {
        Ok(user) => {
            let json_response = UserResponse {
                status: "success".to_string(),
                message: "User created".to_string(),
                user,
            };
            Ok(HttpResponse::Created().json(json_response))
        }
        Err(_) => {
            let error_response = GenericResponse {
                status: "error".to_string(),
                message: "User creation failed".to_string(),
            };

            Ok(HttpResponse::BadRequest().json(error_response))
        }
    }
}
