use api;
use actix_web::{header, Application, HttpMessage, HttpRequest, Method};

pub fn app() -> Application {
    Application::new()
        .resource("/", |r| r.f(index))
        .resource( "/auth", |r| {
            r.method(Method::POST).f(api::auth::register);
            r.method(Method::PATCH).f(api::auth::change_pw);
        })
        .resource( "/auth/change_pw", |r| r.method(Method::POST).f(api::auth::change_pw))
        .resource( "/auth/params",  |r| r.method(Method::GET).f(api::auth::params))
        .resource( "/auth/sign_in", |r| r.method(Method::POST).f(api::auth::sign_in))
        .resource( "/auth/update", |r| r.method(Method::POST).f(api::auth::update))
        .resource( "/items/sync", |r| r.method(Method::POST).f(api::items::sync))
}

static INDEX: &'static str = "For API, see https://standardfile.org";
fn index(request: HttpRequest) -> &'static str {
    info!("Request came in on '/', {:?}", request.headers().get(header::http::AUTHORIZATION));
    INDEX
}

