use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedQuery;
use serde_json;

use db;
use pwdetails;

use super::{ERROR_MISSINGEMAIL,ErrorMsg,Msg};

/**
 * Return the parameters used for password generation.
 *   If user exists, return saved parameters, otherwise generate.
 **/
pub fn params(req: &mut Request) -> IronResult<Response> {
    let res = match req.get_ref::<UrlEncodedQuery>() {
        Err(_) => throw_badrequest(ERROR_MISSINGEMAIL.to_string()),
        Ok(ref hashmap) => {
            let params = hashmap.get(&"email".to_string()).unwrap();
            let val = params.first().unwrap();
            match to_valid_email(val) {
                None => throw_badrequest(ERROR_MISSINGEMAIL.to_string()),
                Some(email) => {
                    let pwmap = get_user_pw_details_or_default(&email);
                    (status::Ok, serde_json::to_string(&pwmap).unwrap())
                }
            }
        }
    };
    Ok(Response::with(res))
}
fn to_valid_email(potential_email: &String) -> Option<String> {
    if potential_email.contains("@") {
        Some(potential_email.clone())
    } else {
        None
    }
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
fn throw_badrequest(msg: String) -> (status::Status, String) {
    (status::BadRequest,
     serde_json::to_string(
         &ErrorMsg {
             error: Msg {
                 message: msg,
                 status: status::Unauthorized.to_u16()
             }
         }).unwrap()
    )
}