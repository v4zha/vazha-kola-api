use super::{db_handler,authorize::Authorize,models::UserProfile};
//password test
#[test]
fn test_pass_gen(){
    let test="Vapp4Ch1Es_LeG4cy";
    assert_eq!(db_handler::check_pass(&db_handler::passwd_gen(test),test),true);
}
//auth test
#[test]
fn test_auth(){
    let secret="Vapp4Ch1Es_LeG4cy";
    let token=Authorize::tokenize(secret.into(),UserProfile{uname:"v4zha".into(),e_mail:"kadavul@v4zha.me".into()});
    let token=format!("Bearer {}",token);
    let auth=Authorize::new(Authorize::parse(token));
    assert_eq!(auth.authorize(secret.into()),true);
}