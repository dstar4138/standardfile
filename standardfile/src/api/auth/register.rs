use bcrypt::{DEFAULT_COST,hash};
use actix_web::{HttpRequest, HttpMessage, StatusCode, AsyncResponder, Error};

use db::get_connection;
use users;
use futures::Future;
use pwdetails::{PasswordDetails, defaults};
use api::{
    UNABLE_TO_REGISTER, ALREADY_REGISTERED,
    ErrorCode, ResultObj, FutureResultObj,
    return_ok, return_err,
};
use super::{
    to_valid_email,
    is_existing_user,
    JwtMsg, encode_user_jwt
};

#[derive(Debug, Serialize, Deserialize)]
struct RegistrationInfo {
    email: String,
    password: String,

    #[serde(flatten)]
    pw_info: PasswordDetails
}

// ERROR CODES
const REGISTRATION_ERROR: ErrorCode = ErrorCode(StatusCode::UNAUTHORIZED, UNABLE_TO_REGISTER);
const DUPLICATE_USER_ERROR: ErrorCode = ErrorCode(StatusCode::UNAUTHORIZED, ALREADY_REGISTERED);

/**
 * Register a user and return a JWT.
 **/
pub fn register(req: HttpRequest) -> FutureResultObj<JwtMsg> {
    req.json()
        .from_err()
        .then(|res : Result<RegistrationInfo, Error>| match res {
            Err(e)   => {
                error!("Error: {}", e);
                Ok(return_err(REGISTRATION_ERROR))
            },
            Ok(info) => Ok(do_register_if_new(info))
        })
        .responder()
}
fn do_register_if_new(info: RegistrationInfo) -> ResultObj<JwtMsg> {
    if let Some(_) = is_existing_user(&info.email) {
        info!("[User: {}], [PWOverrides, {:?}], [Result: already_exists]", &info.email, &info.pw_info);
        return_err(REGISTRATION_ERROR)
    } else {
        info!("[User: {}], [PWOverrides, {:?}], [Result: success]", &info.email, &info.pw_info);
        do_register(info)
    }
}
fn do_register(info: RegistrationInfo) -> ResultObj<JwtMsg> {
    let (potential_email, password) = (info.email, info.password);
    let email = match to_valid_email(&potential_email) {
        None => return return_err(DUPLICATE_USER_ERROR),
        Some(email) => email
    };
    let encrypted_password = hash(&password.as_str(), DEFAULT_COST).expect("Failed to hash password.");
    let pwd = defaults(&info.pw_info);

    let new_user = users::create_new(email, encrypted_password, pwd);

    // Store/Return it.
    let conn = get_connection().expect("Unable to get db connection");
    conn.add_user(&new_user);
    return_ok(encode_user_jwt(&new_user))
}
