use bcrypt::{verify};
use actix_web::{HttpRequest, HttpMessage, StatusCode, AsyncResponder, Error};
use futures::Future;

use backend_core::models::{User};
use api::{
    INVALID_EMAIL_OR_PW,
    FutureResultObj, ResultObj, ErrorCode,
    return_err, return_ok,
};
use super::{
    JwtMsg, encode_user_jwt,
    is_existing_user,
};

#[derive(Debug, Serialize, Deserialize)]
struct SignInRequest {
    email: String,
    password: String,
}

// ERROR CODES
const INVALID_PARAMS: ErrorCode = ErrorCode(StatusCode::UNAUTHORIZED, INVALID_EMAIL_OR_PW);

pub fn sign_in(req: HttpRequest) -> FutureResultObj<JwtMsg> {
    req.json()
        .from_err()
        .then(|res : Result<SignInRequest, Error>| match res {
            Err(e)   => {
                error!("Error: {}", e);
                Ok(return_err(INVALID_PARAMS))
            },
            Ok(info) => Ok(do_sign_in_if_existing(info))
        })
        .responder()
}
fn do_sign_in_if_existing(info: SignInRequest) -> ResultObj<JwtMsg> {
    if let Some(user) = is_existing_user(&info.email) {
        do_sign_in(&user, info)
    } else {
        info!("[User: {}], [Result: invalid_params]", &info.email);
        return_err(INVALID_PARAMS)
    }
}
fn do_sign_in(user: &User, info: SignInRequest) -> ResultObj<JwtMsg> {
    match verify(&info.password.as_str(), &user.encrypted_password.as_str()) {
        Ok(true) => {
            info!("[User: {}], [Result: success]", &info.email);
            return_ok(encode_user_jwt(user))
        },
        _        => {
            info!("[User: {}], [Result: invalid_pw]", &info.email);
            return_err(INVALID_PARAMS)
        }
    }
}
