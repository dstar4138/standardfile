use env;
use util;
use backend_core::models::{User};

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct PasswordDetails {
    pub pw_func: String,
    pub pw_alg: String,
    pub pw_cost: i32,
    pub pw_key_size: i32,
    pub pw_nonce: String,
    pub pw_salt: String,
    pub version: String,
}

impl Default for PasswordDetails {
    fn default() -> PasswordDetails {
        PasswordDetails {
            pw_func: "pbkdf2".to_string(),
            pw_alg : "sha512".to_string(),
            pw_cost: 5_000,
            pw_key_size: 512,
            pw_nonce: "".to_string(),
            pw_salt: "".to_string(),
            version: "002".to_string(),
        }
    }
}

pub fn get_pw_details(user: &User) -> PasswordDetails {
    PasswordDetails {
        pw_func:     user.pw_func.clone(),
        pw_alg:      user.pw_alg.clone(),
        pw_cost:     user.pw_cost.clone(),
        pw_key_size: user.pw_key_size.clone(),
        pw_nonce:    user.pw_nonce.clone(),
        pw_salt:     user.pw_salt.clone(),
        version:     user.version.clone(),
    }
}
pub fn new_pw_details(email: &String) -> PasswordDetails {
    let salt = env::get_pseudo_salt();
    let default = PasswordDetails::default();
    PasswordDetails{
        pw_salt: util::sha1_digest(vec![email, &"SN".to_string(), &salt]),
        ..default
    }
}
