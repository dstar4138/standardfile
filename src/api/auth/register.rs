use iron::prelude::*;
use iron::status;
use bcrypt::{DEFAULT_COST,hash};
use serde_json::Value;

use db;
use users;
use pwdetails;
use models::{User};
use api::{
    UNABLE_TO_REGISTER,ALREADY_REGISTERED,
    encode_error_msg,
    load_json_req_body
};
use super::{
    encode_user_jwt,
    as_valid_email,
    reqmap_to_existing_user,
};

/**
 * Register a user and return a JWT.
 **/
pub fn register(req: &mut Request) -> IronResult<Response> {
    let res = match load_json_req_body(req) {
        Err(_) => encode_error_msg(status::Unauthorized, UNABLE_TO_REGISTER),
        Ok(ref hashmap) => {
            match reqmap_to_existing_user(hashmap) {
                Some(_) => encode_error_msg(status::Unauthorized, ALREADY_REGISTERED),
                None => {
                    // Do the registration
                    match build_register_user_from_reqmap(hashmap) {
                        Err(msg) => encode_error_msg(status::Unauthorized, msg.as_str()),
                        Ok(newuser) => encode_user_jwt(&newuser),
                    }
                }
            }
        }
    };
    Ok(Response::with(res))
}
fn build_register_user_from_reqmap(hashmap: &Value) -> Result<User, String> {
    // The following are REQUIRED params per the spec.
    let email: String = as_valid_email(&attempt_get(&"email".to_string(), hashmap)?).unwrap();
    let password = attempt_get(&"password".to_string(), hashmap)?.as_str().unwrap().to_string();

    // We locally discard what the customer sends us and just hash their pass.
    let encrypted_password = hash(password.as_str(), DEFAULT_COST).unwrap();

    // The default pw details, they can override them though...
    let default_pwd = pwdetails::new_pw_details(&email);
    let pwd = lift_pw_params(hashmap, default_pwd);

    // Create a user struct with these details.
    let new_user = users::create_new(email,encrypted_password,pwd);

    // Store/Return it.
    let conn = db::get_connection();
    db::add_user(&conn,&new_user);
    Ok(new_user)
}
fn attempt_get(key: &String, hashmap: &Value) -> Result<Value,String> {
    match hashmap.get(key) {
        None => Err(format!("Missing valid {}", key)),
        Some(json_val) => Ok(json_val.clone())
    }
}
fn lift_pw_params(hashmap: &Value, default_pwd: pwdetails::PasswordDetails) -> pwdetails::PasswordDetails {
    pwdetails::PasswordDetails {
        pw_func:    hashmap.get("pw_func")    .unwrap_or(&json!(default_pwd.pw_func)).as_str().unwrap().to_string(),
        pw_alg:     hashmap.get("pw_alg")     .unwrap_or(&json!(default_pwd.pw_alg)).as_str().unwrap().to_string(),
        pw_cost:    hashmap.get("pw_cost")    .unwrap_or(&json!(default_pwd.pw_cost)).as_u64().unwrap() as i32,
        pw_key_size:hashmap.get("pw_key_size").unwrap_or(&json!(default_pwd.pw_key_size)).as_u64().unwrap() as i32,
        pw_nonce:   hashmap.get("pw_nonce")   .unwrap_or(&json!(default_pwd.pw_nonce)).as_str().unwrap().to_string(),
        pw_salt:    hashmap.get("pw_salt")    .unwrap_or(&json!(default_pwd.pw_salt)).as_str().unwrap().to_string(),
        version:    hashmap.get("version")    .unwrap_or(&json!(default_pwd.version)).as_str().unwrap().to_string(),
    }
}
