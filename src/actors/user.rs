use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;

use crate::models::user_model::{User, UserDetails};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn add_user(
    conn: &mut PgConnection,
    _full_name: &str,
    _email: &str,
    _password: &str,
) -> Result<UserDetails, DbError> {
    use crate::schema::users::dsl::*;
    let current_time = Utc::now().naive_utc();
    let hashed_password = hash(_password.as_bytes(), DEFAULT_COST)?;
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        full_name: _full_name.to_owned(),
        email: _email.to_owned(),
        password: hashed_password,
        created_at: current_time,
        updated_at: current_time,
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(UserDetails {
        full_name: new_user.full_name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        created_at: new_user.created_at.to_owned(),
        updated_at: new_user.updated_at.to_owned(),
    })
}

pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> Result<UserDetails, DbError> {
    use crate::schema::users::dsl::*;
    // Use the `filter` method from the `FilterDsl` trait to create a query that filters by email
    let user_exists = users
        .filter(email.eq(_email)) // Compare the `email` column with the provided email
        .select(email)
        .first::<String>(conn) // Try to retrieve the email
        .optional()?; // Convert the result to an Option

    match user_exists {
        Some(_) => {
            let user: User = users.filter(email.eq(email)).first(conn)?;
            Ok(UserDetails {
                full_name: user.full_name.to_owned(),
                email: user.email.to_owned(),
                password: user.password.to_owned(),
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
        }
        None => Err(Box::new(diesel::result::Error::NotFound)),
    }
}
