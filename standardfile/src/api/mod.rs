pub mod auth;
pub mod items;

use serde::Serialize;
use futures::Future;
use actix_web::{
    Json,Either,
    StatusCode,Error,
    HttpRequest,HttpMessage,
    HttpResponse,Responder,
    header, Body,
};

use tokens;
use db::StandardFileStorage;

const ERROR_MISSING_EMAIL: &'static str = "Please provide email via GET paramater.";
const UNABLE_TO_REGISTER: &'static str = "Unable to register.";
const ALREADY_REGISTERED: &'static str = "This email is already registered.";
const INVALID_EMAIL_OR_PW: &'static str = "Invalid email or password.";
const INVALID_CREDENTIALS: &'static str = "Invalid login credentials.";

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: Msg
}
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
    status: u16
}

pub struct ErrorCode(StatusCode, &'static str);
pub type ResultObj<T> = Either<Json<T>, ErrorCode>;
pub type FutureResultObj<T> = Box<Future<Item=ResultObj<T>, Error=Error>>;
impl Responder for ErrorCode {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to(self, _req: HttpRequest) -> Result<Self::Item, Self::Error> {
        match self.0 {
            StatusCode::NO_CONTENT =>
                Ok(HttpResponse::new(StatusCode::OK, Body::Empty)),
            _ => HttpResponse::build(self.0).json(
                ErrorMsg {
                    error: Msg {
                        message: self.1.to_string(),
                        status: self.0.as_u16()
                    }
                })
        }
    }
}
pub fn return_err<T:Serialize>(error_code : ErrorCode) -> ResultObj<T> {
    Either::B(error_code)
}
pub fn return_ok<T:Serialize>(json: T) -> ResultObj<T> {
    Either::A(Json(json))
}

fn get_user_agent(req: &HttpRequest) -> String {
    match req.headers().get(header::http::USER_AGENT) {
        None => "".to_string(),
        Some(user_agent) => user_agent.to_str().unwrap_or("").to_string()
    }
}

fn get_current_user_uuid<T:Serialize>(req: &HttpRequest, conn: &Box<StandardFileStorage>) -> Result<String,ResultObj<T>> {
    match get_auth_token_from_header(req) {
        None => Err(return_err(ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS))),
        Some(auth_token) => {
            match tokens::decode_jwt(&auth_token) {
                Err(_) => Err(return_err(ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS))),
                Ok(claims) =>
                    match conn.find_user_by_uuid(&claims.user_uuid) {
                        None => Err(return_err(ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS))),
                        Some(user) =>
                            if claims.pw_hash == tokens::sha256(&user.encrypted_password) {
                                Ok(claims.user_uuid)
                            } else {
                                Err(return_err(ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS)))
                            }
                    }
            }
        }
    }
}

fn get_auth_token_from_header(req: &HttpRequest) -> Option<String> {
    match req.headers().get(header::http::AUTHORIZATION) {
        None => None,
        Some(bearer_auth_token) => {
            let tokenstr = bearer_auth_token.to_str().unwrap_or("");
            match tokenstr.starts_with("Bearer ")  {
                false => None,
                true => Some(tokenstr[7..].to_string())
            }
        }
    }
}
