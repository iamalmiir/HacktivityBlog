use crate::{
    actors::auth::Auth,
    models::user_model::{CreateUser, User},
    utils::helpers::DbPool,
};
use actix_session::Session;
use actix_web::{delete, post, web, HttpResponse, Responder, Result};
use serde_json::json;

#[post("/user/create")]
async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<CreateUser>,
) -> Result<impl Responder> {
    // Creates a new user in the database.
    //
    // # Parameters
    //
    // * `pool`: The database connection pool.
    // * `form`: The user data submitted in the request body.
    //
    // # Returns
    //
    // A `HttpResponse` with a JSON object containing the status, message, and user data if the user was created successfully.
    // If the user could not be created, a `HttpResponse` with a JSON object containing an error message is returned.
    //
    let mut conn = pool.get().unwrap();
    let user: CreateUser = form.into_inner();

    if User::find_user_by_email(&mut conn, &user.email).is_ok() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "User already exists",
        })));
    }

    let created_user = User::add_user(
        &mut conn,
        CreateUser {
            full_name: user.full_name.to_owned(),
            email: user.email.to_owned(),
            password: user.password.to_owned(),
        },
    );

    match created_user {
        Ok(_) => Ok(HttpResponse::Created().json(json!({
            "status": "success",
            "message": "User created",
        }))),

        Err(_) => Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Could not create user",
        }))),
    }
}

#[delete("/user/me")]
async fn delete_user(pool: web::Data<DbPool>, session: Session) -> Result<impl Responder> {
    // Deletes the currently authenticated user from the database.
    //
    // # Parameters
    //
    // * `pool`: The database connection pool.
    // * `session`: The user's session data.
    //
    // # Returns
    //
    // A `HttpResponse` with a JSON object containing the status and message of the operation.
    //
    // # Errors
    //
    // If the user could not be deleted, a `HttpResponse` with a JSON object containing an error message is returned.
    //
    let mut conn = pool.get().unwrap();
    let user_session = Auth::validate_session(&session).unwrap();
    match User::delete_user(&mut conn, &user_session) {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User deleted",
        }))),

        Err(_) => Ok(HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "Could not delete user",
        }))),
    }
}
