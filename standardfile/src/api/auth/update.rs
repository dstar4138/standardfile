use db::{get_connection};
use pwdetails;
use backend_core::models::{User};
use serde_json::Value;

use hyper::StatusCode;
use gotham::state::State;
use gotham::handler::HandlerFuture;
use gotham::http::response::create_response;

use api::{
    INVALID_CREDENTIALS,
    encode_error_msg,
    with_json_body,
    get_current_user_uuid
};

pub fn update(state: State) -> Box<HandlerFuture> {
    println!("UPDATE: Request <=");
    with_json_body(state, |mut state: &State, potential_hashmap| {
        let conn = get_connection().expect("Unable to get db connection");
        let response = match get_current_user_uuid(&state, &conn) {
            Err(err_msg) => err_msg,
            Ok(user_uuid) => match potential_hashmap {
                Err(_) => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_CREDENTIALS),
                Ok(ref hashmap) => {
                    info!("update: {:?}", hashmap);
                    match conn.find_user_by_uuid(&user_uuid) {
                        None => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_CREDENTIALS),
                        Some(user) => {
                            let updated_user = update_pw_params(hashmap, user);
                            conn.update_user(updated_user);
                            create_response(&state, StatusCode::NoContent, None)
                        }
                    }
                }
            }
        };
        println!("UPDATE: Response => {:?}", response);
        response
    })
}

fn update_pw_params(hashmap : &Value, user: User) -> User {
    let defaults = pwdetails::get_pw_details(&user);
    let pw_params = lift_pw_params(hashmap,defaults);
    User {
        pw_func:     pw_params.pw_func.clone(),
        pw_alg:      pw_params.pw_alg.clone(),
        pw_cost:     pw_params.pw_cost.clone(),
        pw_key_size: pw_params.pw_key_size.clone(),
        pw_nonce:    pw_params.pw_nonce.clone(),
        pw_salt:     pw_params.pw_salt.clone(),
        version:     pw_params.version.clone(),
        ..user
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