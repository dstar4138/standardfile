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

use tokens;
use super::ServiceState;
use backend_core::models::{User};
use db::FindUserByEmail;

#[derive(Serialize, Deserialize)]
struct MinimalUser {
    uuid: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
pub struct JwtMsg {
    user: MinimalUser,
    token: String,
}

fn encode_user_jwt(user: &User) -> JwtMsg {
    JwtMsg {
        user: MinimalUser {
            uuid: user.uuid.clone(),
            email: user.email.clone(),
        },
        token: tokens::user_to_jwt(&user).unwrap(),
    }
}

fn to_valid_email(potential_email: &String) -> Option<String> {
    if potential_email.contains("@") {
        Some(potential_email.clone())
    } else {
        None
    }
}
