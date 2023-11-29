use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;

use crate::models::user_model::{CreateUser, User, UserDetails};

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// Adds a new user to the database
///
/// # Parameters
///
/// * `conn` - The database connection
/// * `data` - The user data to create, including their full name, email address, and password
///
/// # Returns
///
/// A `UserDetails` struct containing the details of the newly created user, including their full name, email address, password, and creation and update timestamps
pub fn add_user(conn: &mut PgConnection, data: &CreateUser) -> Result<UserDetails, DbError> {
    use crate::schema::users::dsl::*;
    let current_time = Utc::now().naive_utc();
    let hashed_password = hash(data.password.as_bytes(), DEFAULT_COST)?;
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        full_name: data.full_name.to_owned(),
        email: data.email.to_owned(),
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

/// Find a user by their email address in the database
///
/// # Parameters
///
/// * `conn` - The database connection
/// * `_email` - The email address of the user to find
///
/// # Returns
///
/// A `User` struct if a user with the specified email address was found, or an error if not
pub fn find_user_by_email(conn: &mut PgConnection, user_email: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    // Attempt to find the user by email
    let result = users.filter(email.eq(user_email)).first::<User>(conn)?;

    Ok(User {
        id: result.id,
        full_name: result.full_name.to_owned(),
        email: result.email.to_owned(),
        password: result.password.to_owned(),
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
