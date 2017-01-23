use iron::{Request,Response,IronResult};
use iron::status;
use router::Router;

static INDEX: &'static [u8] = b"For API, see https://standardfile.org";

pub fn handler() -> Router {
    return router!(
        // AUTH -------------------------------------
        index:      any   "/"             => index /*,
        auth:       post  "/auth"         => auth,
        up_auth:    patch "/auth"         => up_auth,
        sign_in:    post  "/auth/sign_in" => sign_in,
        params:     get   "/auth/params"  => params,

        // ITEMS ------------------------------------
        sync:       post   "/items/sync"  => sync  */
    )
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(
        Response::with((status::Ok, INDEX))
    )
}

