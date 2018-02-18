mod params;
mod sign_in;
mod register;

// Export those pesky APIs
pub use self::params::params;
pub use self::sign_in::sign_in;
pub use self::register::register;

static ERROR_MISSINGEMAIL: &'static str = "Please provide email via GET paramater.";
static UNABLE_TO_REGISTER: &'static str = "Unable to register.";
static ALREADY_REGISTERED: &'static str = "This email is already registered.";
static INVALID_EMAIL_OR_PW: &'static str = "Invalid email or password.";

#[derive(Serialize, Deserialize)]
struct ErrorMsg {
    error: Msg
}
#[derive(Serialize, Deserialize)]
struct Msg {
    message: String,
    status: u16
}

#[derive(Serialize, Deserialize)]
struct MinimalUser{
    uuid: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
struct JwtMsg {
    user: MinimalUser,
    token: String,
}

use iron::status;
use serde_json;

fn encode_error_msg(status: status::Status, error: &str) -> (status::Status, String) {
    (status, serde_json::to_string(
         &ErrorMsg {
             error: Msg {
                 message: error.to_string(),
                 status: status.to_u16()
             }
         }).unwrap())
}