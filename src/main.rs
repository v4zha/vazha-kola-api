#[macro_use]
extern crate diesel;
#[path = "./db_mod/db_handler.rs"]
pub mod db_handler;
pub mod error_handler;
#[path = "./db_mod/models.rs"]
pub mod models;
pub mod schema;
use self::models::{LoginUser, NewUser};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result,Responder};
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
    println!("server running on : {}", ip_port);
    HttpServer::new(move || {
        //test-env cors :)
        //use white_list env variable to white_list origins in production
        let cors = Cors::permissive();
        //  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]);
        App::new()
            .wrap(cors)
            .data(db_pool.clone())
            .route("/signup", web::post().to(signup))
            // .route("/disp_data",web::get().to(disp_data))
            .route("/login", web::post().to(login))
    })
    .bind(ip_port)
    .expect("Error binding to Port")
    .run()
    .await
}

async fn signup(
    db_pool: web::Data<DbPool>,
    res: web::Json<NewUser>,
) -> impl Responder {
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let res = db_handler::signup_user(db_conn, res).await;
    match res {
        Err(error_handler::ApiError::DbError(err)) => {
            println!("[DB_ERROR]:\n{}", err);
            web::Json(Response::new("Error adding data".into()))
        }
        _ => {
            let response="successfully signedup".into();
            web::Json(Response::new(response))
        }
    }
}

async fn login(
    db_pool: web::Data<DbPool>,
    res: web::Json<LoginUser>,
) -> impl Responder{
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let resp = db_handler::login_user(db_conn, res).await;
    match resp {
        Ok(LoginResponse::Autherize(val))=>{web::Json(AuthResponse::new(val))},
        Ok(LoginResponse::UserExist(val))=>{
            if val==false{
                web::Json(Response::new("No user Found : )".into()))
            }
            else{
                 web::Json(Response::new("Auth failed :)".into()))
            }},
        Err(err) => {
            println!("[Error]:\n{:?}", err);
            Box::new(web::Json(Response::new("Error processsing request".into())))
        }
    }
}
#[derive(Serialize)]
pub struct Response {
    result: String,
}

impl Response{
    fn new(result: String,authorize:bool) -> Self{
        Self { result ,authorize}
    }
}    
#[derive(Serialize)]
pub struct AuthResponse{
    
}
impl AuthResponse{
    fn new(auth: bool) -> Self{
        Self {authorize:auth}
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
