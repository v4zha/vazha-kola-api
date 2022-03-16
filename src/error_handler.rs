use crate::models::UserProfile;

#[derive(Debug)]
pub enum ApiError {
    ParseError,
    EnvError,
    DbError(diesel::result::Error),
}
pub enum LoginResponse{
    UserExist(bool),
    Authorize(AuthUser),
}
pub struct AuthUser{
    pub authorize:bool,
    pub user:UserProfile,
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> ApiError {
        ApiError::DbError(err)
    }
}
