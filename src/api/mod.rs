pub mod auth;
pub mod items;

use serde_json;
use serde_json::Value;
use tokens;

use mime;
use futures::{Future, Stream};
use hyper::{Body,Headers,StatusCode,Response};
use hyper::header::{UserAgent, Authorization, Bearer};
use gotham::state::{FromState,State};
use gotham::http::response::create_response;

use db::{DbConnection,StandardFileStorage};

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

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct QueryStringExtractor {
    email: String,
}

fn encode_error_msg(state: &State, status: StatusCode, error: &str) -> Response {
    let body = ( serde_json::to_vec(
        &ErrorMsg {
            error: Msg {
                message: error.to_string(),
                status: status.as_u16()
            }
        }).unwrap(), mime::APPLICATION_JSON);
    create_response(state, status, Some(body))
}

fn load_json_req_body(state: &mut State) -> Result<Value,()> {
    match Body::take_from(state).concat2().map(
        |full_body| {
                let body_content = String::from_utf8(full_body.to_vec()).unwrap();
                serde_json::to_value(body_content)
            }).wait() {
        Ok(Ok(json_body)) => Ok(json_body),
        Ok(Err(_)) => Err(()),
        Err(_) => Err(())
    }
}

fn get_user_agent(state: &State) -> String {
    match Headers::borrow_from(state).get::<UserAgent>() {
        None => "".to_string(),
        Some(ref user_agent) => user_agent.to_string()
    }
}

fn get_current_user_uuid(state: &State, conn: &DbConnection) -> Result<String,Response> {
    match Headers::borrow_from(state).get::<Authorization<Bearer>>() {
        None => Err(encode_error_msg(state,StatusCode::Unauthorized, INVALID_CREDENTIALS)),
        Some(ref auth_token) => {
            match tokens::decode_jwt(&auth_token.token) {
                Err(_) => Err(encode_error_msg(state,StatusCode::Unauthorized, INVALID_CREDENTIALS)),
                Ok(claims) =>
                    match conn.find_user_by_uuid(&claims.user_uuid) {
                        None => Err(encode_error_msg(state,StatusCode::Unauthorized, INVALID_CREDENTIALS)),
                        Some(user) =>
                            if claims.pw_hash == tokens::sha256(&user.encrypted_password) {
                                Ok(claims.user_uuid)
                            } else {
                                Err(encode_error_msg(state, StatusCode::Unauthorized, INVALID_CREDENTIALS))
                            }
                    }
            }
        }
    }
}