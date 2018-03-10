use iron::prelude::*;
use iron::status;

use db::{get_connection,StandardFileStorage};
use std::fmt;
use pwdetails;
use models::{User};
use bcrypt::{DEFAULT_COST, hash};
use serde_json::Value;
use api::{
    INVALID_CREDENTIALS,
    encode_error_msg,
    load_json_req_body,
    get_current_user_uuid
};

/**
 *
 */
pub fn change_pw(req: &mut Request) -> IronResult<Response> {
    let conn = get_connection().expect("Unable to get db conn.");
    let res = match get_current_user_uuid(req, &conn) {
        Err(err_msg) => err_msg,
        Ok(user_uuid) => match load_json_req_body(req) {
            Err(_) => encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS),
            Ok(ref hashmap) => {
                info!("change pw: {:?}", hashmap);
                match conn.find_user_by_uuid(&user_uuid) {
                    None => encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS),
                    Some(user) => {
                        let new_pass = get(hashmap, "new_password")?.as_str().unwrap();
                        let new_pass_hash = hash(new_pass, DEFAULT_COST).unwrap();
                        let updated_user = update_pw_params(hashmap, User {
                            encrypted_password: new_pass_hash,
                            ..user
                        })?;
                        conn.update_user(updated_user);
                        (status::NoContent, String::new())
                    }
                }
            }
        }
    };
    Ok(Response::with(res))
}

fn get<'a>(hashmap: &'a Value, key: &str) -> Result<&'a Value,IronError> {
    match hashmap.get(key) {
        None => {
            let msg = encode_error_msg(status::BadRequest, &format!("Missing parameter {}", key));
            Err(IronError::new(fmt::Error, msg))
        },
        Some(val) => Ok(val)
    }
}

fn update_pw_params(hashmap : &Value, user: User) -> Result<User,IronError> {
    let defaults = pwdetails::get_pw_details(&user);
    let pw_params = lift_pw_params(hashmap,defaults);
    Ok(User {
        pw_func:     pw_params.pw_func.clone(),
        pw_alg:      pw_params.pw_alg.clone(),
        pw_cost:     pw_params.pw_cost.clone(),
        pw_key_size: pw_params.pw_key_size.clone(),
        pw_nonce:    pw_params.pw_nonce.clone(),
        pw_salt:     pw_params.pw_salt.clone(),
        version:     pw_params.version.clone(),
        ..user
    })
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