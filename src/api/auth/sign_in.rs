use iron::prelude::*;
use iron::status;
use bodyparser;
use bcrypt::{verify};

use serde_json;
use serde_json::value::Value;

use db;
use tokens;
use models::{User};
use super::{INVALID_EMAIL_OR_PW,JwtMsg,MinimalUser,encode_error_msg};

pub fn sign_in(req: &mut Request) -> IronResult<Response> {
    let res = match load_json_req_body(req) {
        Err(_) => encode_error_msg(status::Unauthorized, INVALID_EMAIL_OR_PW),
        Ok(ref hashmap) => {
            match reqmap_to_existing_user(hashmap) {
                None => encode_error_msg(status::Unauthorized, INVALID_EMAIL_OR_PW),
                Some(user) => {
                    // Do the registration
                    match verify_password_from_params(hashmap, &user) {
                        false => encode_error_msg(status::Unauthorized, INVALID_EMAIL_OR_PW),
                        true => {
                            let user_jwt = JwtMsg {
                                user: MinimalUser {
                                    uuid: user.uuid.clone(),
                                    email: user.email.clone(),
                                },
                                token: tokens::user_to_jwt(&user).unwrap(),
                            };
                            (status::Ok, serde_json::to_string(&user_jwt).unwrap())
                        },
                    }
                }
            }
        }
    };
    Ok(Response::with(res))
}
fn verify_password_from_params(hashmap: &Value, user: &User) -> bool {
    let password = hashmap.get(&"password".to_string()).unwrap();
    verify(&password.as_str().unwrap(), &user.encrypted_password.as_str()).unwrap()
}
fn reqmap_to_existing_user(hashmap: &Value) -> Option<User> {
   match as_valid_email(hashmap.get(&"email".to_string()).unwrap()) {
       None => None,
       Some(email) => {
           let conn = db::get_connection();
           db::find_user_by_email(&conn, &email)
       }
   }
}
fn load_json_req_body(req: &mut Request) -> Result<Value,()> {
    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => Ok(json_body),
        Ok(None) => Err(()),
        Err(_) => Err(())
    }
}
fn as_valid_email(potential_email: &Value) -> Option<String> {
    if potential_email.is_string() {
        let val = potential_email.as_str().unwrap().to_string();
        to_valid_email(&val)
    } else {
        None
    }
}
fn to_valid_email(potential_email: &String) -> Option<String> {
    if potential_email.contains("@") {
        Some(potential_email.clone())
    } else {
        None
    }
}