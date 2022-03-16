use super::models::UserProfile;
use jsonwebtoken::{encode, Header,EncodingKey};

pub fn tokenize(user:UserProfile,key:String)->String{
    encode(&Header::default(),&user,&EncodingKey::from_secret(key.as_ref())).unwrap()
}
