use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;
use serde_json;

use db;
use pwdetails;

use api::{ERROR_MISSINGEMAIL,encode_error_msg};
use super::to_valid_email;

/**
 * Return the parameters used for password generation.
 *   If user exists, return saved parameters, otherwise generate.
 **/
pub fn params(req: &mut Request) -> IronResult<Response> {
    let res = match req.get_ref::<UrlEncodedQuery>() {
        Err(_) => encode_error_msg(status::BadRequest,ERROR_MISSINGEMAIL),
        Ok(ref hashmap) => {
            let params = hashmap.get(&"email".to_string()).unwrap();
            let val = params.first().unwrap();
            match to_valid_email(val) {
                None => encode_error_msg(status::BadRequest,ERROR_MISSINGEMAIL),
                Some(email) => {
                    let pwmap = get_user_pw_details_or_default(&email);
                    (status::Ok, serde_json::to_string(&pwmap).unwrap())
                }
            }
        }
    };
    Ok(Response::with(res))
}
fn get_user_pw_details_or_default(email: &String) -> pwdetails::PasswordDetails {
    let conn = db::get_connection();
    match db::find_user_by_email(&conn, email) {
        None =>
            pwdetails::new_pw_details(email),
        Some(user) =>
            pwdetails::get_pw_details(&user)
    }
}