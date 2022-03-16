use super::schema::vkala_users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize,Serialize};
use std::convert::From;
#[derive(Queryable, Debug,Clone)]
pub struct VkalaUsers {
    pub id: i32,
    pub uname: String,
    pub passwd:String,
    pub e_mail:String,
}
#[derive(Insertable, Deserialize)]
#[table_name = "vkala_users"]
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
#[derive(Deserialize)]
pub struct LoginUser {
    pub uname: String,
    pub passwd:String,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserProfile{
    pub uname:String,
    pub e_mail:String,
}
impl From<VkalaUsers> for UserProfile{
    fn from(user:VkalaUsers)->Self{
        Self{
            uname:user.uname,
            e_mail:user.e_mail,
        }
    }
}
