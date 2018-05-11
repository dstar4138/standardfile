use api;
use actix_web::{http, App, HttpRequest};
use actix_web::middleware::identity::{IdentityService};
use actix_web::middleware::Logger;

pub fn app(state: api::ServiceState) -> App<api::ServiceState> {
    App::with_state(state)
        .resource("/", |r| r.f(index))
        .resource( "/auth", |r| {
            r.method(http::Method::POST).with2(api::auth::register);
            r.method(http::Method::PATCH).with3(api::auth::change_pw);
        })
        .resource( "/auth/change_pw", |r| r.method(http::Method::POST).with3(api::auth::change_pw))
        .resource( "/auth/params",  |r| r.method(http::Method::GET).with2(api::auth::params))
        .resource( "/auth/sign_in", |r| r.method(http::Method::POST).with2(api::auth::sign_in))
        .resource( "/auth/update", |r| r.method(http::Method::POST).with3(api::auth::update))
        .resource( "/items/sync", |r| r.method(http::Method::POST).with3(api::items::sync))
        .middleware(Logger::new("%t,[IP: %a],[Time: %D ms],[Status: %s],[Auth: %{Authorization}i] - %r"))
        .middleware(IdentityService::new(
            api::identity::TokenIdentityPolicy
        ))
}

static INDEX: &'static str = "For API, see https://standardfile.org";
fn index(_request: HttpRequest<api::ServiceState>) -> &'static str {
    INDEX
}

