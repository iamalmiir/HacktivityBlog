use crate::models::{self, user_model::NewUser};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_user(
    conn: &mut PgConnection,
    _user: &models::user_model::NewUser,
) -> Result<models::user_model::NewUser, DbError> {
    use crate::schema::users::dsl::*;
    let current_time = Utc::now().naive_utc();
    let hashed_password = hash(_user.password.as_bytes(), DEFAULT_COST)?;
    let new_user = models::user_model::User {
        id: uuid::Uuid::new_v4(),
        full_name: _user.full_name.to_owned(),
        email: _user.email.to_owned(),
        password: hashed_password,
        created_at: current_time,
        updated_at: current_time,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(NewUser {
        full_name: new_user.full_name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
    })
}

pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> Result<bool, DbError> {
    use crate::schema::users::dsl::*;

    // Use the `filter` method from the `FilterDsl` trait to create a query that filters by email
    let user_exists = users
        .filter(email.eq(_email)) // Compare the `email` column with the provided email
        .select(email)
        .first::<String>(conn) // Try to retrieve the email
        .optional()?; // Convert the result to an Option

    match user_exists {
        Some(_) => Ok(true),
        None => Err(Box::new(diesel::result::Error::NotFound)),
    }
}
