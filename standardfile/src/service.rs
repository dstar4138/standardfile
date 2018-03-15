use api;
use api::QueryStringExtractor;

use gotham::state::State;
use gotham::router::Router;
use gotham::router::builder::*;
use hyper::{Response,StatusCode};

static INDEX: &'static [u8] = b"For API, see https://standardfile.org";

pub fn router() -> Router {
    build_simple_router(|route| {
        // INDEX ------------------------------------
        route.get_or_head("/").to(index);

        // AUTH -------------------------------------
        route
            .associate("/auth",|assoc| {
                assoc.post().to(api::auth::register);
                assoc.patch().to(api::auth::change_pw);
            });
        route
            .get("/auth/params")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(api::auth::params);
        route
            .post("/auth/sign_in")
            .to(api::auth::sign_in);
        route
            .post("/auth/change_pw")
            .to(api::auth::change_pw);
        route
            .post("/auth/update")
            .to(api::auth::update);

        // ITEMS ------------------------------------
        route
            .post("/items/sync")
            .to(api::items::sync);
    })
}

fn index(state: State) -> (State, Response) {
    (state, Response::new()
             .with_status(StatusCode::Ok)
             .with_body(INDEX))
}
