use super::schema::vkola_users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize,Serialize};
use std::convert::From;
use::std::time::SystemTime;
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
//Claims model
//Password not required after Login <Authorization verified by JWT : )>
#[derive(Debug,Serialize,Deserialize)]
pub struct UserProfile{
    pub uname:String,
    pub e_mail:String,
    exp:u64,
}
impl UserProfile{
    //Gen exp for duration of 1hr ie 3600
    fn gen_exp()->u64{
        	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()+3600
    }
    pub fn new(uname:String,e_mail:String)->Self{
        Self{uname,e_mail,exp:UserProfile::gen_exp()}
    }
}
impl From<VkolaUsers> for UserProfile{
    fn from(user:VkolaUsers)->Self{
        UserProfile::new(user.uname,user.e_mail)
    }
}
