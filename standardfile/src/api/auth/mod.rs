mod params;
mod sign_in;
mod register;
mod change_pw;
mod update;

// Export those pesky APIs
pub use self::params::params;
pub use self::sign_in::sign_in;
pub use self::register::register;
pub use self::change_pw::change_pw;
pub use self::update::update;

use tokens;
use backend_core::models::{User};
use serde_json::Value;
use serde_json;
use db::{get_connection};

use mime;
use hyper::{StatusCode,Response};
use gotham::state::State;
use gotham::http::response::create_response;

#[derive(Serialize, Deserialize)]
struct MinimalUser {
    uuid: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
struct JwtMsg {
    user: MinimalUser,
    token: String,
}

fn encode_user_jwt(state: &State, user: &User) -> Response{
    let user_jwt = JwtMsg {
        user: MinimalUser {
            uuid: user.uuid.clone(),
            email: user.email.clone(),
        },
        token: tokens::user_to_jwt(&user).unwrap(),
    };
    let body = Some((serde_json::to_vec(&user_jwt).unwrap(),mime::APPLICATION_JSON));
    create_response(state,StatusCode::Ok, body)
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

fn reqmap_to_existing_user(hashmap: &Value) -> Option<User> {
   match as_valid_email(hashmap.get(&"email".to_string()).unwrap()) {
       None => None,
       Some(email) => {
           let conn = get_connection().expect("Unable to get db conn.");
           conn.find_user_by_email(&email)
       }
   }
}