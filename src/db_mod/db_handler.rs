use super::{
    error_handler::{ApiError, LoginResponse},
    models::{LoginUser, VkalaUsers, NewUser},
    schema::vkala_users,
    PoolConn,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use vkala_users::dsl::*;

pub async fn signup_user(pg_conn: PoolConn, data: web::Json<NewUser>) -> Result<(), ApiError> {
    let user = NewUser::new(&data.uname, &data.passwd,&data.e_mail);
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
        Ok(val) => Ok(LoginResponse::Autherize(check_pass(val,data.passwd))),
        Err(err) => Err(ApiError::DbError(err)),
    }
}
