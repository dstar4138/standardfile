use iron::prelude::*;
use iron::status;
use serde_json;
use urlencoded::UrlEncodedQuery;

use db;
use env;
use users;

static ERROR_MISSINGEMAIL: &'static [u8] = b"Please provide email via GET paramater.";

/**
 * Return the parameters used for password generation.
 *   If user exists, return saved parameters, otherwise generate.
 **/
pub fn params(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        Err(_) =>
            Ok(Response::with((status::BadRequest,ERROR_MISSINGEMAIL))),
        Ok(ref hashmap) => {
            match to_valid_email(hashmap.get(&"email".to_string()).unwrap()) {
                None =>
                    Ok(Response::with((status::BadRequest,ERROR_MISSINGEMAIL))),
                Some(email) => {
                    let dbpath = env::get_database_path();
                    let conn = db::create_connection(dbpath.as_str());
                    let pwmap = match db::find_user_by_email(&conn, &email) {
                        None => users::new_pw_details(&email, &env::get_pseudo_salt()),
                        Some(user) => users::get_pw_details(&user)
                    };
                    Ok(Response::with((status::Ok,
                                       serde_json::to_string(&pwmap).unwrap().as_str())))
                }
            }
        }
    }
}
fn to_valid_email(params: &Vec<String>) -> Option<String> {
    match params.first() {
        None => None,
        Some(val) => {
            if val.contains("@") {
                Some(val.clone())
            } else {
                None
            }
        }
    }
}