use bcrypt::{DEFAULT_COST,hash};
use serde_json::Value;

use hyper::StatusCode;
use gotham::state::State;
use gotham::handler::HandlerFuture;

use db::get_connection;
use users;
use pwdetails;
use backend_core::models::{User};
use api::{
    UNABLE_TO_REGISTER,ALREADY_REGISTERED,
    encode_error_msg,
    with_json_body
};
use super::{
    encode_user_jwt,
    as_valid_email,
    reqmap_to_existing_user,
    get_pw_params
};

/**
 * Register a user and return a JWT.
 **/
pub fn register(state: State) -> Box<HandlerFuture> {
    info!("Request <=");
    with_json_body(state, |state : &State, potential_hashmap| {
        let response = match potential_hashmap {
            Err(_) => encode_error_msg(state, StatusCode::Unauthorized, UNABLE_TO_REGISTER),
            Ok(ref hashmap) => {
                info!("\t <= {:?}", hashmap);
                match reqmap_to_existing_user(hashmap) {
                    Some(_) => encode_error_msg(state, StatusCode::Unauthorized, ALREADY_REGISTERED),
                    None =>
                        // Do the registration
                        match build_register_user_from_reqmap(hashmap) {
                            Err(msg) => encode_error_msg(state, StatusCode::Unauthorized, msg.as_str()),
                            Ok(newuser) => encode_user_jwt(state, &newuser),
                        }

                }
            }
        };
        info!("Response => {:?}", response);
        response
    })
}
fn build_register_user_from_reqmap(hashmap: &Value) -> Result<User, String> {
    // The following are REQUIRED params per the spec.
    let email: String = as_valid_email(&attempt_get(&"email".to_string(), hashmap)?).unwrap();
    let password = attempt_get(&"password".to_string(), hashmap)?.as_str().unwrap().to_string();

    // We locally discard what the customer sends us and just hash their pass.
    let encrypted_password = hash(password.as_str(), DEFAULT_COST).unwrap();

    // The default pw details, they can override them though...
    let default_pwd = pwdetails::new_pw_details(&email);
    let pwd = get_pw_params(hashmap, default_pwd);

    // Create a user struct with these details.
    let new_user = users::create_new(email,encrypted_password,pwd);

    // Store/Return it.
    let conn = get_connection().expect("Unable to get db connection");
    conn.add_user(&new_user);
    Ok(new_user)
}
fn attempt_get(key: &String, hashmap: &Value) -> Result<Value,String> {
    match hashmap.get(key) {
        None => Err(format!("Missing valid {}", key)),
        Some(json_val) => Ok(json_val.clone())
    }
}
