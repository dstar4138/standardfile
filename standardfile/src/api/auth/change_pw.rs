use db::{get_connection, StandardFileStorage};
use backend_core::models::{User};
use bcrypt::{DEFAULT_COST, hash};
use pwdetails::{
    PasswordDetails,
    HasPasswordDetails,
    defaults_with_override
};
use util;

use actix_web::{HttpRequest, HttpMessage, StatusCode, AsyncResponder, Error};
use futures::{Future, IntoFuture};

use api::{
    INVALID_CREDENTIALS,
    get_current_user_uuid,
    return_err, return_ok, ErrorCode,
    FutureResultObj, ResultObj,
};
use super::{
    encode_user_jwt, JwtMsg
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    new_password: String,

    #[serde(flatten)]
    pw_info: PasswordDetails
}

const UPDATE_ERROR: ErrorCode = ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS);

pub fn change_pw(req: HttpRequest) -> FutureResultObj<JwtMsg> {
    let conn = get_connection().expect("Unable to get db connection");
    let user_uuid = match get_current_user_uuid(&req, &conn) {
        Err(err_msg) => return Box::new(Ok(err_msg).into_future()),
        Ok(user_uuid) => user_uuid
    };
    req.json()
        .from_err()
        .then(move |res : Result<ChangePasswordRequest, Error>| match res {
            Err(e) => {
                error!("Error: {}",e);
                Ok(return_err(UPDATE_ERROR))
            },
            Ok(request) => {
                let conn = get_connection().expect("Unable to get db connection");
                Ok(do_change_pw(request, &user_uuid, &conn))
            }
        })
        .responder()
}

pub fn do_change_pw(request: ChangePasswordRequest, user_uuid: &String, conn: &Box<StandardFileStorage>) -> ResultObj<JwtMsg> {
    if let Some(user) = conn.find_user_by_uuid(user_uuid) {
        info!("[UserUuid: {}], [Result: success]", user_uuid);
        let new_pass_hash = hash(&request.new_password.as_str(), DEFAULT_COST).unwrap();
        let pw_params = defaults_with_override(&request.pw_info, &user.to_password_details());
        let current_time = util::current_time();
        let new_user = User {
            updated_at: current_time,
            encrypted_password: new_pass_hash,
            pw_func: pw_params.get_pw_func(),
            pw_alg: pw_params.get_pw_alg(),
            pw_cost: pw_params.get_pw_cost(),
            pw_key_size: pw_params.get_pw_key_size(),
            pw_nonce: pw_params.get_pw_nonce(),
            pw_salt: pw_params.get_pw_salt(),
            version: pw_params.get_version(),
            ..user
        };
        let ret = encode_user_jwt(&new_user);
        conn.update_user(new_user);
        return_ok(ret)
    } else {
        info!("[UserUuid: {}], [Request: {:?}], [Result: could_not_find_user]", user_uuid, request);
        return_err(UPDATE_ERROR)
    }
}
