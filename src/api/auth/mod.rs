mod params;
mod sign_in;
mod register;
mod change_pw;
mod update;

// Export those pesky APIs
pub use self::params::params;
pub use self::sign_in::sign_in;
pub use self::register::register;
pub use self::change_pw::change_pw;
pub use self::update::update;

use db;
use tokens;
use models::{User};
use iron::status;
use serde_json::Value;
use serde_json;

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

fn encode_user_jwt(user: &User) -> (status::Status, String) {
    let user_jwt = JwtMsg {
        user: MinimalUser {
            uuid: user.uuid.clone(),
            email: user.email.clone(),
        },
        token: tokens::user_to_jwt(&user).unwrap(),
    };
    (status::Ok, serde_json::to_string(&user_jwt).unwrap())
}

fn as_valid_email(potential_email: &Value) -> Option<String> {
    if potential_email.is_string() {
        let val = potential_email.as_str().unwrap().to_string();
        to_valid_email(&val)
    } else {
        None
    }
}
fn to_valid_email(potential_email: &String) -> Option<String> {
    if potential_email.contains("@") {
        Some(potential_email.clone())
    } else {
        None
    }
}

fn reqmap_to_existing_user(hashmap: &Value) -> Option<User> {
   match as_valid_email(hashmap.get(&"email".to_string()).unwrap()) {
       None => None,
       Some(email) => {
           let conn = db::get_connection();
           db::find_user_by_email(&conn, &email)
       }
   }
}
