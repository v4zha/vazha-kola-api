use actix_http::ResponseError;
use std::fmt;
use super::models::UserProfile;
#[derive(Debug)]
//Error Variants
pub enum ApiError {
    ParseError,
    EnvError,
    DbError(diesel::result::Error),
    WebError(actix_web::Error),
}
//Reponse Enum used in Login Handler
//return User_exists and user authorization status 
pub enum LoginResponse{
    UserExist(bool),
    Authorize(AuthUser),
}
//AuthUser Struct ->Part of LoginResponse
//Gets User authoraization status and details
pub struct AuthUser{
    pub authorize:bool,
    pub user:UserProfile,
}

//From trait implementation
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
//Display trait implementation
//also required for ResponseError implementation
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
//Required for FromRequest implementation
impl ResponseError for ApiError{
}
