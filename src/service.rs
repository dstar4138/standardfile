//use futures;
//use hyper::{Error, Get};
//use hyper::header::ContentLength;
//use hyper::server::{Service, Request, Response};
//use hyper::status::StatusCode;

use hyper::Get;
use hyper::status::StatusCode;
use hyper::server::{Request, Response};
use hyper::uri::RequestUri::AbsolutePath;

static INDEX: &'static [u8] = b"For API, see https://standardfile.org";

/* POST hyper version 0.10:
pub struct StandardFileService;
impl Service for StandardFileService {

    type Error = Error;
    type Request = Request;
    type Respnse = Response;
    type Future = futures::Finished<Response, Error>; 

    fn call(&self, req: Request) -> Self::Future {
        futures::finished(
            match (req.method(), req.path()) {
                (&Get, "/") => {
                    Response::new()
                        .with_header(ContentLength(INDEX.len() as u64))
                        .with_body(INDEX)
                },

                // TODO: flesh out API.

                _ => Response::new().with_status(StatusCode::NotFound)
            })
    }
}
*/

macro_rules! try_return(
    ($e:expr) => {{
        match $e {
            Ok(v) => v,
            Err(e) => { println!("Error: {}", e); return; }
        }
    }}
);
pub fn handle(req: Request, mut res: Response) {
    match req.uri {
        AbsolutePath(ref path) => match (&req.method, &path[..]) {
            (&Get, "/") => {
                try_return!(res.send(INDEX));
                return;
            },
            
            //TODO: flesh out API.

            _ => {
                *res.status_mut() = StatusCode::NotFound;
                return;
            }
        },
        _ => {
            return;
        }
    }
}

