use iron::prelude::*;
use iron::status;
use router::Router;

use api;

static INDEX: &'static [u8] = b"For API, see https://standardfile.org";

pub fn handler() -> Router {
    return router!(
        index:      any   "/"             => index,
        echo:       get   "/echo/:q"      => echo,
        // AUTH -------------------------------------
        params:     get   "/auth/params"  => api::auth::params,
        auth:       post  "/auth"         => api::auth::register,
        sign_in:    post  "/auth/sign_in" => api::auth::sign_in,
/*        up_auth:    patch "/auth"         => up_auth, */

        // ITEMS ------------------------------------
        sync:       post   "/items/sync"  => api::items::sync
    )
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(
        Response::with((status::Ok, INDEX))
    )
}

fn echo(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>()
            .unwrap().find("q").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}
