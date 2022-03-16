#[macro_use]
extern crate diesel;
extern crate argon2;
#[path="./db_mod/auth.rs"]
pub mod auth;
use auth::tokenize;
#[path = "./db_mod/db_handler.rs"]
pub mod db_handler;
pub mod error_handler;
#[path = "./db_mod/models.rs"]
pub mod models;
pub mod schema;
use self::models::{LoginUser, NewUser};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use actix_web_httpauth::headers::authorization::{Authorization, Basic};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use dotenv::dotenv;
use error_handler::LoginResponse;
use r2d2;
use serde::Serialize;
use std::env;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PoolConn = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let white_list=env::var("ORIGINS").unwrap();
    let db_pool = init_dbpool().await;
    let port = env::var("PORT").expect("Error parsing Port Var");
    let host = env::var("HOST").expect("Error parsing HOST Var");
    let ip_port = format!("{}:{}", host, port);
    let secret=env::var("SECRET_KEY").expect("Error parsing SECRET_KEY Var");
    println!("server running on : {}", ip_port);
    HttpServer::new(move || {
        //test-env cors :)
        //use white_list env variable to white_list origins in production
        let cors = Cors::permissive();
        //  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]);
        App::new()
            .wrap(cors)
            .data(db_pool.clone())
            .data(secret.clone())
            .route("/signup", web::post().to(signup))
            // .route("/disp_data",web::get().to(disp_data))
            .route("/login", web::post().to(login))
            .route("/test",web::get().to(test_user))
    })
    .bind(ip_port)
    .expect("Error binding to Port")
    .run()
    .await
}

async fn signup(db_pool: web::Data<DbPool>, res: web::Json<NewUser>) -> impl Responder {
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let res = db_handler::signup_user(db_conn, res).await;
    match res {
        Err(error_handler::ApiError::DbError(err)) => {
            println!("[DB_ERROR]:\n{}", err);
            web::Json(Response::new("Error adding data".into()))
        }
        _ => {
            let response = "successfully signedup".into();
            web::Json(Response::new(response))
        }
    }
}

async fn login(db_pool: web::Data<DbPool>, res: web::Json<LoginUser>,secret:String) -> impl Responder {
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let resp = db_handler::login_user(db_conn, res).await;
    match resp {
        Ok(LoginResponse::Authorize(val)) => {
            if val.authorize{
            web::Json(AuthResponse::new("Auth result".into(),tokenize(val.user,secret),val.authorize))
            }
            else{
                web::Json(AuthResponse::new("Auth failed :)".into(),"".into(),false))
            }
        }
        Ok(LoginResponse::UserExist(val)) => {
            if val == false {
                web::Json(AuthResponse::new("No user Found : )".into(),"".into(), false))
            } else {
                web::Json(AuthResponse::new("Auth failed :)".into(),"".into() ,false))
            }
        }
        Err(err) => {
            println!("[Error]:\n{:?}", err);
            web::Json(AuthResponse::new("Error processsing request".into(), "".into(),false))
        }
    }
}
async fn test_usr()->impl Responder {
    web::Json(Response::new("You are allowed to view this page",into()))
}
#[derive(Serialize)]
pub struct Response {
    result: String,
}

impl Response {
    fn new(result: String) -> Self {
        Self { result }
    }
}
#[derive(Serialize)]
pub struct AuthResponse {
    result: String,
    token:String,
    authorize: bool,
}

impl AuthResponse {
    fn new(result: String,token:String,authorize: bool) -> Self {
    Self { result,token,authorize }
    }
}
pub async fn init_dbpool() -> DbPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let conn_manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Error building Dbconnector")
}
