use super::models::UserProfile;
use std::collections::HashSet;
use actix_web::{FromRequest,HttpRequest,dev};
use futures::future::{Ready,ok};
use jsonwebtoken::{encode,decode,DecodingKey, Header,EncodingKey,Algorithm,Validation};
use super::error_handler::ApiError;

#[derive(Debug)]
pub struct Authorize{
    key:String,
}
    impl  Authorize{
    pub fn new(key:String)->Self{
        Self{key}
    }
    pub fn tokenize(&self,user:UserProfile)->String{
        encode(&Header::default(),&user,&EncodingKey::from_secret(&self.key.as_ref())).unwrap()
    }

    pub fn authorize(&self,token:String)->bool{
        let mut validation =Validation::new(Algorithm::HS256);
        validation.validate_exp=false;
        validation.required_spec_claims=HashSet::new();
        let dec=decode::<UserProfile>(&token, &DecodingKey::from_secret(self.key.as_ref()),&validation);
        match dec{
            Ok(val)=>{println!("{:?}",val.claims);true},
            Err(_)=>false
        }
    }
}
impl FromRequest for Authorize{
    type Error=ApiError;
    type Future=Ready<Result<Self,Self::Error>>;
    type Config=();
    fn from_request(_req:&HttpRequest,_payload:&mut dev::Payload)->Self::Future{
        let x=Authorize::new("halle_looyah".into());
        ok(x)
    }
}