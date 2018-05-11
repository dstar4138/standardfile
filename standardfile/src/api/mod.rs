pub mod auth;
pub mod items;
pub mod identity;
mod errors;

use actix_web::{
    HttpRequest, HttpMessage,
    http,
};

use db::{DBAddr};

pub struct ServiceState {
    pub db: DBAddr
}

fn get_user_agent(req: &HttpRequest<ServiceState>) -> String {
    match req.headers().get(http::header::USER_AGENT) {
        None => "".to_string(),
        Some(user_agent) => user_agent.to_str().unwrap_or("").to_string()
    }
}
