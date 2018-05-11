use actix_web::{HttpResponse,State,Query,AsyncResponder,FutureResponse,Either,ResponseError};
use futures::Future;

use api::{
    ServiceState,
    to_valid_email,
    errors::SFError
};
use db::FindUserByEmail;
use pwdetails::{HasPasswordDetails,new_pw_details};

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
                .and_then(move |res|
                    if let Ok(Some(user)) = res {
                        Ok(user.to_password_details())
                    } else {
                        Ok(new_pw_details(&email))
                    })
                .and_then(move |pwd| Ok(HttpResponse::Ok().json(pwd)))
                .responder()
        )
    } else {
        Either::B(SFError::MissingEmail.error_response())
    }
}
