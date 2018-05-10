use bcrypt::{DEFAULT_COST,hash};
use actix_web::{
    FutureResponse, HttpResponse, AsyncResponder,
    ResponseError,
    Json, State, Either
};

use db::AddUser;
use users;
use futures::Future;
use pwdetails::{PasswordDetails, defaults};
use api::{
    ServiceState,
    errors::SFError,
};
use super::{
    to_valid_email,
    encode_user_jwt
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationInfo {
    email: String,
    password: String,

    #[serde(flatten)]
    pw_info: PasswordDetails
}

/**
 * Register a user and return a JWT.
 **/
pub fn register(info: Json<RegistrationInfo>, state: State<ServiceState>) ->
    Either<FutureResponse<HttpResponse>, HttpResponse>
{
    let (potential_email, password) = (info.email.clone(), info.password.clone());
    let email = match to_valid_email(&potential_email) {
        None => return Either::B(SFError::AlreadyRegistered.error_response()),
        Some(email) => email
    };
    let encrypted_password = hash(&password.as_str(), DEFAULT_COST).expect("Failed to hash password.");
    let pwd = defaults(&info.pw_info);

    let new_user = users::create_new(email, encrypted_password, pwd);

    // Store/Return it.
    Either::A(state.db.send(
        AddUser {
            user: new_user.clone()
        }).from_err()
        .and_then(move |res| match res {
            Err(_) => Ok(SFError::AlreadyRegistered.error_response()),
            Ok(_) => Ok(HttpResponse::Ok().json(encode_user_jwt(&new_user)))
        })
        .responder())
}
