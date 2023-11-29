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

// pub fn find_user_by_email(conn: &mut PgConnection, _email: &str) -> Result<User, DbError> {
//     use crate::schema::users::dsl::*;

//     // Attempt to find the user by email
//     match users.filter(email.eq(_email)).first::<User>(conn) {
//         Ok(user) => Ok(User {
//             id: user.id,
//             full_name: user.full_name.to_owned(),
//             email: user.email.to_owned(),
//             password: user.password.to_owned(),
//             created_at: user.created_at,
//             updated_at: user.updated_at,
//         }),
//         // If the user doesn't exist, return a 404 error
//         Err(diesel::result::Error::NotFound) => Err(Box::new(diesel::result::Error::NotFound)),
//         Err(e) => Err(Box::new(e)), // Handle any other database error
//     }
// }
pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    // Attempt to find the user by email
    let result = users.filter(email.eq(email)).first::<User>(conn)?;

    Ok(User {
        id: result.id,
        full_name: result.full_name.to_owned(),
        email: result.email.to_owned(),
        password: result.password.to_owned(),
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
