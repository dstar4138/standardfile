mod params;
mod sign_in;
mod register;

// Export those pesky APIs
pub use self::params::params;
pub use self::sign_in::sign_in;
pub use self::register::register;

use db;
use tokens;
use models::{User};
use iron::prelude::*;
use iron::status;
use bodyparser;
use serde_json::Value;
use serde_json;

static ERROR_MISSINGEMAIL: &'static str = "Please provide email via GET paramater.";
static UNABLE_TO_REGISTER: &'static str = "Unable to register.";
static ALREADY_REGISTERED: &'static str = "This email is already registered.";
static INVALID_EMAIL_OR_PW: &'static str = "Invalid email or password.";

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: Msg
}
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
    status: u16
}

#[derive(Serialize, Deserialize)]
struct MinimalUser{
    uuid: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
struct JwtMsg {
    user: MinimalUser,
    token: String,
}

fn encode_error_msg(status: status::Status, error: &str) -> (status::Status, String) {
    (status, serde_json::to_string(
         &ErrorMsg {
             error: Msg {
                 message: error.to_string(),
                 status: status.to_u16()
             }
         }).unwrap())
}

fn encode_user_jwt(user: &User) -> (status::Status, String) {
    let user_jwt = JwtMsg {
        user: MinimalUser {
            uuid: user.uuid.clone(),
            email: user.email.clone(),
        },
        token: tokens::user_to_jwt(&user).unwrap(),
    };
    (status::Ok, serde_json::to_string(&user_jwt).unwrap())
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