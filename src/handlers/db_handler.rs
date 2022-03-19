use crate::models::UserProfile;

use super::{
    error_handler::{ApiError, AuthUser, LoginResponse},
    models::{LoginUser, NewUser, VkolaUsers},
    schema::vkola_users,
    PoolConn,
};
use actix_web::web;
use argon2;
use diesel::{result, ExpressionMethods,RunQueryDsl};
use diesel::QueryDsl;
use rand::Rng;
use vkola_users::dsl::*;

//SignUp user
//connect to DB
//prevents duplicate entry by checking uname conflict [uname set as unique in DB Schema]
pub async fn signup_user(pg_conn: PoolConn, data: web::Json<NewUser>) -> Result<(), ApiError> {
    let user = NewUser::new(&data.uname, &passwd_gen(&data.passwd), &data.e_mail);
    let res = diesel::insert_into(vkola_users::table)
        .values(&user)
        .on_conflict(uname)
        .do_nothing()
        .execute(&pg_conn);
    match res {
        Err(err) => Err(ApiError::DbError(err)),
        Ok(_val) => Ok(()),
    }
}

//Login User
//validating password by hashing and fetching DB entry
pub async fn login_user(
    pg_conn: PoolConn,
    data: web::Json<LoginUser>,
) -> Result<LoginResponse, ApiError> {
    let u_name: String = format!("{}", data.uname);
    let result = get_users(pg_conn, u_name).await;
    match result {
        Ok(val) if val.len() == 0 => Ok(LoginResponse::UserExist(false)),
        Ok(val) => Ok(LoginResponse::Authorize(AuthUser {
            user: UserProfile::from(val[0].clone()),
            authorize: check_pass(&val[0].passwd, &data.passwd),
        })),
        Err(err) => Err(ApiError::DbError(err)),
    }
}
//Check Password Hash to verify user
pub fn check_pass(pass: &str, resp: &str) -> bool {
    let resp = resp.as_bytes();
    let val = argon2::verify_encoded(&pass, resp);
    val.unwrap()
}
//Generate Password Hash to store in DB
pub fn passwd_gen(pass: &str) -> String {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = argon2::Config::default();
    let hash = argon2::hash_encoded(pass.as_bytes(), &salt, &config);
    hash.unwrap()
}
//Get User detials based on user_name
async fn get_users(pg_conn: PoolConn, u_name: String) -> Result<Vec<VkolaUsers>, result::Error> {
    let result = vkola_users::dsl::vkola_users
        .filter(vkola_users::dsl::uname.eq(u_name))
        .load::<VkolaUsers>(&pg_conn);
    result
}
