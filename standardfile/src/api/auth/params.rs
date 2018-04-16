use pwdetails::{PasswordDetails,HasPasswordDetails,new_pw_details};
use actix_web::{HttpRequest,StatusCode};

use db::get_connection;
use api::{
    ERROR_MISSING_EMAIL,
    ResultObj, ErrorCode,
    return_err, return_ok
};
use super::to_valid_email;

// ERROR CODES
const BAD_REQUEST: ErrorCode = ErrorCode(StatusCode::BAD_REQUEST, ERROR_MISSING_EMAIL);

pub fn params(request: HttpRequest) -> ResultObj<PasswordDetails> {
    match request.query().get("email") {
        None => return_err(BAD_REQUEST),
        Some(potential_email) => match to_valid_email(&potential_email.to_string()) {
            None => return_err(BAD_REQUEST),
            Some(email) => {
                info!("[User: {}]", email);
                return_ok(get_user_pw_details_or_default(&email))
            }
        }
    }
}
fn get_user_pw_details_or_default(email: &String) -> PasswordDetails {
    let conn = get_connection().expect("Unable to get db connection.");
    match conn.find_user_by_email(email) {
        None => new_pw_details(email),
        Some(user) => user.to_password_details()
    }
}
