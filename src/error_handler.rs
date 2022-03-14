#[derive(Debug)]
pub enum ApiError {
    ParseError,
    EnvError,
    DbError(diesel::result::Error),
}
pub enum LoginResponse{
    UserExist(bool),
    Autherize(bool),
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> ApiError {
        ApiError::DbError(err)
    }
}
