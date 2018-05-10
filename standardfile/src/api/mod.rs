pub mod auth;
pub mod items;
pub mod identity;
mod errors;

use serde::Serialize;
use futures::prelude::*;
use actix::prelude::*;
use actix_web::{
    HttpRequest, HttpMessage, AsyncResponder,
    http, HttpResponse, Error, Either,
    FutureResponse, ResponseError,
};

use tokens;
use db::{DBAddr,FindUserByUUID};

pub struct ServiceState {
    pub db: DBAddr
}

fn get_user_agent(req: &HttpRequest<ServiceState>) -> String {
    match req.headers().get(http::header::USER_AGENT) {
        None => "".to_string(),
        Some(user_agent) => user_agent.to_str().unwrap_or("").to_string()
    }
}
/*
fn get_current_user_uuid<F>(req: &HttpRequest<ServiceState>, callback: F) -> Either<FutureResponse<HttpResponse>,HttpResponse>
    where
        F : FnOnce(String) -> FutureResponse<HttpResponse,errors::SFError>,
        F : 'static
{
    if let Some(auth_token) = get_auth_token_from_header(req) {
        if let Ok(claims) = tokens::decode_jwt(&auth_token) {
            return Either::A(req.state().db
                .send(FindUserByUUID {
                    uuid: claims.user_uuid.clone()
                })
                .from_err()
                .and_then(|res| match res {
                    Err(_) => Err(errors::SFError::InvalidCredentials),
                    Ok(potential_user) => match potential_user {
                        None => Err(errors::SFError::InvalidCredentials),
                        Some(user) =>
                            if claims.pw_hash == tokens::sha256(&user.encrypted_password) {
                                Ok(claims.user_uuid)
                            } else {
                                Err(errors::SFError::InvalidCredentials)
                            }
                    }
                })
                .then(move |res| match res {
                    Ok(user_uuid) => callback(user_uuid),
                    Err(err) => Ok(err.error_response()).into(),
                })
                .flatten()
                .responder())
        }
    }

    Either::B(errors::SFError::InvalidCredentials.error_response())
}

fn get_auth_token_from_header(req: &HttpRequest<ServiceState>) -> Option<String> {
    match req.headers().get(http::header::AUTHORIZATION) {
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
*/