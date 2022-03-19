use super::models::UserProfile;
use actix_web::{FromRequest,HttpRequest,dev};
use futures::future::{Ready,ok};
use jsonwebtoken::{encode,decode,DecodingKey, Header,EncodingKey,Validation};
use super::error_handler::ApiError;

#[derive(Debug)]
pub struct Authorize{
    token:String,
}
    impl  Authorize{
    pub fn new(token:String)->Self{
        Self{token}
    }
    //tokenize content
    pub fn tokenize(key:String,user:UserProfile)->String{
        encode(&Header::default(),&user,&EncodingKey::from_secret(key.as_ref())).unwrap()
    }
    //authorize token :)
    pub fn authorize(&self,key:String)->bool{
        let dec=decode::<UserProfile>(&self.token, &DecodingKey::from_secret(key.as_ref()),&Validation::default());
        match dec{
            Ok(val)=>{println!("{:?}",val.claims);true},
            Err(_)=>false
        }
    }
    pub fn parse(bearer:String)->String{
        bearer.replace("Bearer ","")
    }
}
//Implement FromRequest to get bearerToken from HttpRequest
impl FromRequest for Authorize{
    type Error=ApiError;
    type Future=Ready<Result<Self,Self::Error>>;
    type Config=();
    fn from_request(req:&HttpRequest,_payload:&mut dev::Payload)->Self::Future{
        let auth=req.headers().get("Authorization").unwrap().to_str().unwrap();
        let token=Authorize::parse(auth.into());
        ok(Authorize::new(token))
    }
}