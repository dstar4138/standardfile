use actix_web::{HttpResponse, ResponseError, Json, State, Either, AsyncResponder, FutureResponse};
use bcrypt::{verify};
use futures::Future;

use api::{
    ServiceState,
    errors::SFError,
    models::JwtMsg,
};
use db::FindUserByEmail;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    email: Option<String>,
    password: Option<String>,
}

pub fn sign_in(req: Json<SignInRequest>, state: State<ServiceState>) -> Either<FutureResponse<HttpResponse>, HttpResponse>  {
    match (req.email.clone(), req.password.clone()) {
        (None, _) => Either::B(SFError::InvalidEmailOrPassword.error_response()),
        (_, None) => Either::B(SFError::InvalidEmailOrPassword.error_response()),
        (Some(email), Some(password)) =>
            Either::A(
                state.db
                    .send(FindUserByEmail { email })
                    .from_err()
                    .and_then(move |res|
                        if let Ok(Some(user)) = res {
                            match verify(password.as_str(), user.encrypted_password.as_str()) {
                                Ok(true) => Ok(HttpResponse::Ok().json(JwtMsg::from(&user))),
                                _ => Ok(SFError::InvalidEmailOrPassword.error_response())
                            }
                        } else {
                            Ok(SFError::InvalidEmailOrPassword.error_response())
                        })
                    .responder())
    }
}
