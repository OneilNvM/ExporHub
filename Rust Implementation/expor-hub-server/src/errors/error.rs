use diesel::r2d2;
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
    #[error("database pool error occurred")]
    R2D2Error(diesel::r2d2::Error),
}

impl std::convert::From<r2d2::Error> for AccountCreationError {
    fn from(value: r2d2::Error) -> Self {
        Self::R2D2Error(value)
    }
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

#[derive(Error, Debug)]
pub enum CommentCreationError {
    #[error("the comment must have at least 1 character")]
    EmptyComment,
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum ReplyCreationError {
    #[error("the reply must have at least 1 character")]
    EmptyReply,
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error("zero rows were deleted")]
    ZeroRowsDeleted,
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum UpdateError {
    #[error("there was an error when parsing the date: {0}")]
    DateParseError(String),
    #[error("an error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Invalid login credentials provided")]
    InvalidCredentials,
    #[error("An error occurred during the database transaction: {0}")]
    DatabaseError(diesel::result::Error),
}
