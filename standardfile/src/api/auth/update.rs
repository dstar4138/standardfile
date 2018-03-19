use db::{get_connection};
use backend_core::models::{User};
use serde_json::Value;
use pwdetails::HasPasswordDetails;

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
use super::get_pw_params;

pub fn update(state: State) -> Box<HandlerFuture> {
    info!("Request <=");
    with_json_body(state, |state: &State, potential_hashmap| {
        let conn = get_connection().expect("Unable to get db connection");
        let response = match (
            get_current_user_uuid(&state, &conn),
            potential_hashmap
        ) {
            (Err(err_msg), _) => err_msg,
            (_, Err(_)) => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_CREDENTIALS),
            (Ok(ref user_uuid), Ok(ref hashmap)) => {
                match conn.find_user_by_uuid(user_uuid) {
                    None => encode_error_msg(&state, StatusCode::Unauthorized, INVALID_CREDENTIALS),
                    Some(user) => {
                        let updated_user = update_pw_params(hashmap, user);
                        conn.update_user(updated_user);
                        create_response(&state, StatusCode::NoContent, None)
                    }
                }
            }
        };
        info!("Response => {:?}", response);
        response
    })
}

fn update_pw_params(hashmap : &Value, user: User) -> User {
    let pw_params = get_pw_params(hashmap, user.to_password_details());
    User {
        pw_func:     pw_params.get_pw_func(),
        pw_alg:      pw_params.get_pw_alg(),
        pw_cost:     pw_params.get_pw_cost(),
        pw_key_size: pw_params.get_pw_key_size(),
        pw_nonce:    pw_params.get_pw_nonce(),
        pw_salt:     pw_params.get_pw_salt(),
        version:     pw_params.get_version(),
        ..user
    }
}