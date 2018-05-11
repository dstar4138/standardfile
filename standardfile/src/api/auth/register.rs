use actix_web::{
    FutureResponse, HttpResponse, AsyncResponder,
    ResponseError,
    Json, State, Either
};
use bcrypt::{DEFAULT_COST,hash};
use futures::Future;

use api::{
    ServiceState,
    to_valid_email,
    errors::SFError,
    models::JwtMsg,
};
use db::AddUser;
use pwdetails::{PasswordDetails, defaults};
use users;

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
    Either::A(
        state.db
        .send(AddUser { user: new_user.clone() })
        .from_err()
        .and_then(move |res| match res {
            Err(_) => Ok(SFError::AlreadyRegistered.error_response()),
            Ok(_) => Ok(HttpResponse::Ok().json(JwtMsg::from(&new_user)))
        })
        .responder()
    )
}
