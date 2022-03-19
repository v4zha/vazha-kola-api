use actix_http::ResponseError;
use std::fmt;
use super::models::UserProfile;
#[derive(Debug)]
pub enum ApiError {
    ParseError,
    EnvError,
    DbError(diesel::result::Error),
    WebError(actix_web::Error),
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
impl From<actix_web::Error> for ApiError{
    fn from(err:actix_web::Error)->ApiError{
        ApiError::WebError(err)
    }
}
impl fmt::Display for ApiError{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        let result={
            match &self{
                ApiError::ParseError=>"Parse error".into(),
                ApiError::EnvError=>"Env varable error".into(),
                ApiError::DbError(err)=>format!("Db_Error:\n{}",err),
                ApiError::WebError(err)=>format!("Server_Error:\n{}",err),
            }
        };
        write!(f,"{}",&result)
    }
}
impl ResponseError for ApiError{
}
