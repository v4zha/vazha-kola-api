use super::{
    error_handler::{ApiError, LoginResponse},
    models::{LoginUser, VkalaUsers, NewUser},
    schema::vkala_users,
    PoolConn,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use vkala_users::dsl::*;
use argon2;
use rand::Rng;

pub async fn signup_user(pg_conn: PoolConn, data: web::Json<NewUser>) -> Result<(), ApiError> {
    let user = NewUser::new(&data.uname, &passwd_gen(&data.passwd),&data.e_mail);
    let res = diesel::insert_into(vkala_users::table)
        .values(&user)
        .on_conflict(uname)
        .do_nothing()
        .execute(&pg_conn);
    match res {
        Err(err) => Err(ApiError::DbError(err)),
        Ok(_val) => Ok(()),
    }
}

pub async fn login_user(
    pg_conn: PoolConn,
    data: web::Json<LoginUser>,
) -> Result<LoginResponse, ApiError> {
    let u_name: String = format!("{}", data.uname);
    let result = vkala_users::dsl::vkala_users
        .filter(vkala_users::dsl::uname.eq(u_name))
        .load::<VkalaUsers>(&pg_conn);
    match result {
        Ok(val) if val.len() == 0 => Ok(LoginResponse::UserExist(false)),
        Ok(val) => Ok(LoginResponse::Autherize(check_pass(&val[0].passwd,&data.passwd))),
        Err(err) => Err(ApiError::DbError(err)),
    }
}

fn check_pass(resp:&str,pass:&str)->bool{
    println!("{} >>\n {}",&pass,resp);
    let resp=resp.as_bytes();
    let val=argon2::verify_encoded(&pass, resp);
    val.unwrap()
}
fn passwd_gen(pass:&str)->String{
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = argon2::Config::default();
    let hash=argon2::hash_encoded(pass.as_bytes(),&salt,&config);
    hash.unwrap()
}   