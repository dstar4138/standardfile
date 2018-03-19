use mime;
use pwdetails::{PasswordDetails,HasPasswordDetails,new_pw_details};
use serde_json;
use hyper::{StatusCode,Response};
use gotham::state::{FromState, State};
use gotham::http::response::create_response;

use db::get_connection;
use api::{
    ERROR_MISSINGEMAIL,
    encode_error_msg,
    QueryStringExtractor
};

use super::to_valid_email;

pub fn params(mut state: State) -> (State, Response) {
    info!("Request <=");
    let query_param = QueryStringExtractor::take_from(&mut state);
    let response = match to_valid_email(&query_param.email) {
        None =>
            encode_error_msg(&state, StatusCode::BadRequest, ERROR_MISSINGEMAIL),
        Some(email) => {
            let pwmap = get_user_pw_details_or_default(&email);
            let content = serde_json::to_vec(&pwmap).unwrap();
            let body = (content, mime::APPLICATION_JSON);
            create_response(&state, StatusCode::Ok, Some(body))
        }
    };
    info!("Response => {:?}", response);
    (state, response)
}
fn get_user_pw_details_or_default(email: &String) -> PasswordDetails {
    let conn = get_connection().expect("Unable to get db connection.");
    match conn.find_user_by_email(email) {
        None => new_pw_details(email),
        Some(user) => user.to_password_details()
    }
}