use iron::status;
use iron::prelude::*;

pub fn sync(req: &mut Request) -> IronResult<Response> {
    let res = (status::Ok,"");
    Ok(Response::with(res))
}