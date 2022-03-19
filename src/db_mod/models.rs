use super::schema::vkola_users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize,Serialize};
use std::convert::From;
//VkalaUsers model used in generating Scheme
//Model used for querying DB
#[derive(Queryable, Debug,Clone)]
pub struct VkolaUsers {
    pub id: i32,
    pub uname: String,
    pub passwd:String,
    pub e_mail:String,
}
//Model for Inserting into DB as id not req.
#[derive(Insertable, Deserialize)]
#[table_name = "vkola_users"]
pub struct NewUser {
    pub uname: String,
    pub passwd:String,
    pub e_mail:String,
}
impl NewUser {
    pub fn new(uname: &str,passwd:&str,email:&str) -> Self {
        Self {
            uname: uname.into(),
            passwd:passwd.into(),
            e_mail:email.into(),
        }
    }
}
//Used by LoginHandler 
#[derive(Deserialize)]
pub struct LoginUser {
    pub uname: String,
    pub passwd:String,
}
//Used by Authorization Handler 
//Password not required after Login <Authorization verified by JWT : )>
#[derive(Debug,Serialize,Deserialize)]
pub struct UserProfile{
    pub uname:String,
    pub e_mail:String,
}
impl From<VkolaUsers> for UserProfile{
    fn from(user:VkolaUsers)->Self{
        Self{
            uname:user.uname,
            e_mail:user.e_mail,
        }
    }
}
