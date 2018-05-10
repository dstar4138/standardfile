use db::{UpdateUser, UserUpdateChange};
use bcrypt::{DEFAULT_COST, hash};
use pwdetails::{PasswordDetails};
use actix_web::{
    HttpRequest, HttpResponse,
    FutureResponse, AsyncResponder,
    Json, State, Either, ResponseError,
};
use actix_web::middleware::identity::RequestIdentity;
use futures::{Future};

use api::{
    errors::SFError,
    ServiceState
};
use super::{
    encode_user_jwt,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    new_password: String,

    #[serde(flatten)]
    pw_info: PasswordDetails
}

pub fn change_pw(req: HttpRequest<ServiceState>, info: Json<ChangePasswordRequest>, state: State<ServiceState>) -> Either<FutureResponse<HttpResponse>, HttpResponse> {
    let user_uuid = match req.identity() {
        Some(user_uuid) => user_uuid.to_string(),
        None => return Either::B(SFError::InvalidCredentials.error_response()),
    };
    let new_pass_hash = match hash(&info.new_password.as_str(), DEFAULT_COST) {
        Ok(pass_hash) => pass_hash.to_string(),
        _ => return Either::B(SFError::InvalidCredentials.error_response()),
    };
    let user_change = UserUpdateChange {
        encrypted_password: Some(new_pass_hash),
        pw_func: info.pw_info.pw_func.clone(),
        pw_alg: info.pw_info.pw_alg.clone(),
        pw_cost: info.pw_info.pw_cost.clone(),
        pw_key_size: info.pw_info.pw_key_size.clone(),
        pw_nonce: info.pw_info.pw_nonce.clone(),
        pw_salt: info.pw_info.pw_salt.clone(),
        version: info.pw_info.version.clone(),
        ..UserUpdateChange::default()
    };
    Either::A(
        state.db.send(
            UpdateUser {
                uuid: user_uuid.clone(),
                user: user_change
            })
            .from_err()
            .and_then(|res| match res {
                Err(_) => Ok(SFError::InvalidCredentials.error_response()),
                Ok(new_user) => Ok(HttpResponse::Ok().json(encode_user_jwt(&new_user)))
            })
            .responder())
}
