use env;
use util;

const DEFAULT_PW_FUNC : &str = "pbkdf2";
const DEFAULT_PW_ALG  : &str = "sha512";
const DEFAULT_PW_COST : i32 = 5_000;
const DEFAULT_PW_KEY_SIZE : i32 = 512;
const DEFAULT_VERSION : &str = "002";

pub trait HasPasswordDetails {
    fn get_pw_func(&self) -> String;
    fn get_pw_alg(&self) -> String;
    fn get_pw_cost(&self) -> i32;
    fn get_pw_key_size(&self) -> i32;
    fn get_pw_nonce(&self) -> String;
    fn get_pw_salt(&self) -> String;
    fn get_version(&self) -> String;
    fn to_password_details(&self) -> PasswordDetails;
}

#[derive(Serialize,Deserialize,Clone,Debug,PartialEq,Eq)]
pub struct PasswordDetails {
    pub pw_func:  Option<String>,
    pub pw_alg:   Option<String>,
    pub pw_cost:     Option<i32>,
    pub pw_key_size: Option<i32>,
    pub pw_nonce: Option<String>,
    pub pw_salt:  Option<String>,
    pub version:  Option<String>,
}

impl Default for PasswordDetails {
    fn default() -> PasswordDetails {
        PasswordDetails {
            pw_func:     Some(DEFAULT_PW_FUNC.to_string()),
            pw_alg :     Some(DEFAULT_PW_ALG.to_string()),
            pw_cost:     Some(DEFAULT_PW_COST),
            pw_key_size: Some(DEFAULT_PW_KEY_SIZE),
            pw_nonce:    None,
            pw_salt:     None,
            version:     Some(DEFAULT_VERSION.to_string()),
        }
    }
}

pub fn new_pw_details(email: &String) -> PasswordDetails {
    let salt = env::get_pseudo_salt();
    let default = PasswordDetails::default();
    PasswordDetails{
        pw_salt: Some(util::sha1_digest(vec![email, &"SN".to_string(), &salt])),
        ..default
    }
}

impl HasPasswordDetails for PasswordDetails {
    fn get_pw_func(&self) -> String {
        if self.pw_func.is_some() {
            self.pw_func.clone().unwrap()
        } else {
            DEFAULT_PW_FUNC.to_string()
        }
    }
    fn get_pw_alg(&self) -> String {
        if self.pw_alg.is_some() {
            self.pw_alg.clone().unwrap()
        } else {
           DEFAULT_PW_ALG.to_string()
        }
    }
    fn get_pw_cost(&self) -> i32 {
        self.pw_cost.unwrap_or(DEFAULT_PW_COST)
    }
    fn get_pw_key_size(&self) -> i32 {
        self.pw_key_size.unwrap_or(DEFAULT_PW_KEY_SIZE)
    }
    fn get_pw_nonce(&self) -> String {
        if self.pw_nonce.is_some() {
            self.pw_nonce.clone().unwrap()
        } else {
            String::new()
        }
    }
    fn get_pw_salt(&self) -> String {
        if self.pw_salt.is_some() {
            self.pw_salt.clone().unwrap()
        } else {
            String::new()
        }
    }
    fn get_version(&self) -> String {
        if self.version.is_some() {
            self.version.clone().unwrap()
        } else {
            DEFAULT_VERSION.to_string()
        }
    }
    fn to_password_details(&self) -> PasswordDetails {
        self.clone()
    }
}