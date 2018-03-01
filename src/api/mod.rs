pub mod auth;
pub mod items;

use bodyparser;
use iron::status;
use iron::headers::*;
use iron::prelude::*;
use serde_json;
use serde_json::Value;
use tokens;
use db;

static ERROR_MISSINGEMAIL: &'static str = "Please provide email via GET paramater.";
static UNABLE_TO_REGISTER: &'static str = "Unable to register.";
static ALREADY_REGISTERED: &'static str = "This email is already registered.";
static INVALID_EMAIL_OR_PW: &'static str = "Invalid email or password.";
static INVALID_CREDENTIALS: &'static str = "Invalid login credentials.";

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: Msg
}
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
    status: u16
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

fn load_json_req_body(req: &mut Request) -> Result<Value,()> {
    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => Ok(json_body),
        Ok(None) => Err(()),
        Err(_) => Err(())
    }
}

fn get_user_agent(req: &mut Request) -> Option<String> {
    match req.headers.get::<UserAgent>() {
        None => None,
        Some(&UserAgent(ref user_agent)) => Some(user_agent.clone())
    }
}

fn get_current_user_uuid(req: &mut Request) -> Result<String,(status::Status,String)> {
    match req.headers.get::<Authorization<Bearer>>() {
        None =>Err(encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS)),
        Some(ref auth_token) => {
            match tokens::decode_jwt(&auth_token.token) {
                Err(_) => Err(encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS)),
                Ok(claims) => {
                    let conn = db::get_connection();
                    match db::find_user_pw_hash_by_uuid(&conn, &claims.user_uuid) {
                        None => Err(encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS)),
                        Some(user_pw_hash) =>
                            if claims.pw_hash == tokens::sha256(&user_pw_hash) {
                                Ok(claims.user_uuid)
                            } else {
                                Err(encode_error_msg(status::Unauthorized, INVALID_CREDENTIALS))
                            }
                    }
                }
            }
        }
    }
}