use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountCreationError {
    #[error("'{0}' is an invalid username")]
    InvalidUsername(String),
    #[error("'{0}' is an invalid email")]
    InvalidEmail(String),
    #[error("the provided password was invalid")]
    InvalidPassword,
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("invalid login credentials provided")]
    InvalidCredentials,
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum ProjectCreationError {
    #[error("the project name '{0}' is not long enough")]
    InvalidProjectName(String),
    #[error("the description '{0}' is not long enough")]
    InvalidDescription(String),
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}