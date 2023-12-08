use crate::models::user_model::{CreateUser, User};
use actix_web::Result;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use validator::Validate;

type DbError = Box<dyn std::error::Error + Send + Sync>;

impl User {
    /// Add a new user to the database
    ///
    /// # Parameters
    ///
    /// * `conn` - The database connection
    /// * `data` - The user data to create, including their full name, email address, and password
    ///
    /// # Returns
    ///
    /// A `UserDetails` struct containing the details of the newly created user, including their full name, email address, password, and creation and update timestamps
    pub fn add_user(conn: &mut PgConnection, data: CreateUser) -> Result<String, DbError> {
        data.validate()?;

        use crate::schema::users::dsl::*;
        let current_time = Utc::now().naive_utc();
        // Insert the new user into the database
        let new_user = diesel::insert_into(users)
            .values(User {
                id: uuid::Uuid::new_v4(),
                full_name: data.full_name,
                email: data.email,
                password: hash(data.password.as_bytes(), DEFAULT_COST)?,
                created_at: current_time,
                updated_at: current_time,
            })
            .execute(conn)?;

        if new_user == 1 {
            Ok("OK".to_string())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not create user",
            )))
        }
    }

    /// Find a user by their email address in the database
    ///
    /// # Parameters
    ///
    /// * `conn` - The database connection
    /// * `user_email` - The email address of the user to find
    ///
    /// # Returns
    ///
    /// A `User` struct if a user with the specified email address was found, or an error if not
    pub fn find_user_by_email(conn: &mut PgConnection, user_email: &str) -> Result<User, DbError> {
        use crate::schema::users::dsl::*;

        // Attempt to find the user by email
        let result = users.filter(email.eq(user_email)).first::<User>(conn)?;

        Ok(result)
    }

    /// Deletes a user from the database based on their email address
    ///
    /// # Parameters
    ///
    /// * `conn` - The database connection
    /// * `user_email` - The email address of the user to delete
    ///
    /// # Returns
    ///
    /// A `String` containing the email address of the deleted user
    pub fn delete_user(conn: &mut PgConnection, user_email: &str) -> Result<String, DbError> {
        use crate::schema::users::dsl::*;
        // Delete the user from the database
        let user_deleted = diesel::delete(users.filter(email.eq(user_email))).execute(conn)?;

        // Return the email address of the deleted user
        match user_deleted {
            0 => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not delete user",
            ))),
            _ => Ok(user_email.to_string()),
        }
    }
}
