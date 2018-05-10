use pwdetails::{HasPasswordDetails,new_pw_details};
use actix_web::{HttpResponse, State,Query,AsyncResponder,FutureResponse,Either,ResponseError};
use futures::Future;

use db::FindUserByEmail;
use api::{ServiceState,errors::SFError};
use super::to_valid_email;

#[derive(Deserialize,Debug)]
pub struct UserSelection {
    email: Option<String>,
}

pub fn params(info: Query<UserSelection>, state: State<ServiceState>) -> Either<FutureResponse<HttpResponse>,HttpResponse> {
    if let Some(email) = info.email.clone() {
        if let None = to_valid_email(&email) {
            return Either::B(SFError::MissingEmail.error_response());
        }

        Either::A(
            state.db
                .send(FindUserByEmail { email: email.clone() })
                .from_err()
                .and_then(move |res| match res {
                    Err(_) => Ok(new_pw_details(&email)),
                    Ok(result) => match result {
                        None => Ok(new_pw_details(&email)),
                        Some(user) => Ok(user.to_password_details())
                    }
                })
                .and_then(move |pwd| Ok(HttpResponse::Ok().json(pwd)))
                .responder()
        )
    } else {
        Either::B(SFError::MissingEmail.error_response())
    }
}
