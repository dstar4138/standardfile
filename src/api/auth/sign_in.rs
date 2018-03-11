use bcrypt::{verify};
use serde_json::Value;

use gotham::state::State;
use hyper::{StatusCode,Response};

use models::{User};
use api::{
    INVALID_EMAIL_OR_PW,
    encode_error_msg,
    load_json_req_body
};
use super::{
    encode_user_jwt,
    reqmap_to_existing_user,
};

pub fn sign_in(mut state: State) -> (State,Response) {
    let response = match load_json_req_body(&mut state) {
        Err(_) => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_EMAIL_OR_PW),
        Ok(ref hashmap) => {
            match reqmap_to_existing_user(hashmap) {
                None => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_EMAIL_OR_PW),
                Some(user) => {
                    // Do the registration
                    match verify_password_from_params(hashmap, &user) {
                        false => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_EMAIL_OR_PW),
                        true  => encode_user_jwt(&state, &user),
                    }
                }
            }
        }
    };
    (state,response)
}
fn verify_password_from_params(hashmap: &Value, user: &User) -> bool {
    let password = hashmap.get(&"password".to_string()).unwrap();
    verify(&password.as_str().unwrap(), &user.encrypted_password.as_str()).unwrap()
}

