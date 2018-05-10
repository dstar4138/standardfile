use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result, http};
use actix_web::error::Error;
use actix_web::middleware::{Response};
use actix_web::middleware::identity::{IdentityPolicy, Identity};
use futures::Future;
use futures::future::{ok as FutOk};

use super::{ServiceState};
use tokens;
use db::FindUserByUUID;

pub struct TokenIdentity {
    uuid: Option<String>,
}
impl Default for TokenIdentity {
    fn default() -> Self {
        TokenIdentity { uuid : None }
    }
}
impl Identity for TokenIdentity {
    fn identity(&self) -> Option<&str> {
        self.uuid.as_ref().map(|s| s.as_ref())
    }

    fn remember(&mut self, value: String) {
        self.uuid = Some(value);
    }

    fn forget(&mut self) {
        self.uuid = None;
    }

    fn write(&mut self, resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}

pub struct TokenIdentityPolicy;
impl IdentityPolicy<ServiceState> for TokenIdentityPolicy {
    type Identity = TokenIdentity;
    type Future = Box<Future<Item = Self::Identity, Error = Error>>;

    fn from_request(&self, req: &mut HttpRequest<ServiceState>) -> Self::Future {
        if let Some(auth_token) = get_auth_token_from_header(req) {
            if let Ok(claims) = tokens::decode_jwt(&auth_token) {
                return Box::new(req.state().db.send(FindUserByUUID {
                    uuid: claims.user_uuid.clone()
                })
                .from_err()
                .and_then(|res| {
                    if let Ok(Some(user)) = res {
                        if claims.pw_hash == tokens::sha256(&user.encrypted_password) {
                            return Ok(TokenIdentity { uuid: Some(claims.user_uuid) });
                        }
                    }

                    Ok(TokenIdentity::default())
                }));
            }
        }

        // No identity given in this request, or claims are invalid.
        Box::new(FutOk(TokenIdentity::default()))
    }
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