use iron::prelude::*;
use iron::status;
use bodyparser;
use std::io::Read;
use urlencoded::UrlEncodedQuery;
use bcrypt::{DEFAULT_COST,hash,verify};

use serde_json;
use serde_json::value::Value;

use db;
use users;
use tokens;
use pwdetails;
use models::{User};

static ERROR_MISSINGEMAIL: &'static str = "Please provide email via GET paramater.";
static UNABLE_TO_REGISTER: &'static str = "Unable to register.";
static ALREADY_REGISTERED: &'static str = "This email is already registered.";

// Limit our read buffer size to try and avoid memory consumption issues.
// All "expected" values are known length limits (besides email address).
const READ_BUFFER_SIZE: usize = 1024;

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
struct JwtMsg {
    jwt: String
}

/**
 * Return the parameters used for password generation.
 *   If user exists, return saved parameters, otherwise generate.
 **/
pub fn params(req: &mut Request) -> IronResult<Response> {
    match req.get_ref::<UrlEncodedQuery>() {
        Err(_) =>
            Ok(Response::with((status::BadRequest,ERROR_MISSINGEMAIL))),
        Ok(ref hashmap) => {
            let params = hashmap.get(&"email".to_string()).unwrap();
            let val = params.first().unwrap();
            match to_valid_email(val) {
                None =>
                    Ok(Response::with((status::BadRequest,ERROR_MISSINGEMAIL))),
                Some(email) => {
                    let pwmap = get_user_pw_details_or_default(&email);
                    let json_map = serde_json::to_string(&pwmap).unwrap();
                    Ok(Response::with((status::Ok, json_map.as_str())))
                }
            }
        }
    }
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
fn get_user_pw_details_or_default(email: &String) -> pwdetails::PasswordDetails {
    let conn = db::get_connection();
    match db::find_user_by_email(&conn, email) {
        None =>
            pwdetails::new_pw_details(email),
        Some(user) =>
            pwdetails::get_pw_details(&user)
    }
}


/**
 * Register a user and return a JWT.
 **/
pub fn register(req: &mut Request) -> IronResult<Response> {
    let res = match load_json_req_body(req) {
        Err(_) => throw_unauthorized(UNABLE_TO_REGISTER.to_string()),
        Ok(ref hashmap) => {
            match reqmap_to_existing_user(hashmap) {
                Some(_) => throw_unauthorized(ALREADY_REGISTERED.to_string()),
                None => {
                    // Do the registration
                    match build_register_user_from_reqmap(hashmap) {
                        Err(msg) => throw_unauthorized(msg),
                        Ok(newuser) => {
                            let user_jwt = JwtMsg {
                                jwt: tokens::user_to_jwt(&newuser).unwrap(),
                            };
                            (status::Ok, serde_json::to_string(&user_jwt).unwrap())
                        },
                    }
                }
            }
        }
    };
    Ok(Response::with(res))
}
fn load_json_req_body(req: &mut Request) -> Result<Value,()> {
    let json_body = req.get::<bodyparser::Json>();
    match json_body {
        Ok(Some(json_body)) => Ok(json_body),
        Ok(None) => Err(()),
        Err(err) => Err(())
    }
}
fn throw_unauthorized(msg: String) -> (status::Status, String) {
    (status::Unauthorized, // Throw a 401 on failures.
     serde_json::to_string(
         &ErrorMsg {
             error: Msg {
                 message: msg,
                 status: status::Unauthorized.to_u16()
             }
         }).unwrap()
    )
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
fn build_register_user_from_reqmap(hashmap: &Value) -> Result<User, String> {
    // The following are REQUIRED params per the spec.
    let email: String = as_valid_email(&attempt_get(&"email".to_string(), hashmap)?).unwrap();
    let password = attempt_get(&"password".to_string(), hashmap)?.as_str().unwrap().to_string();

    // We locally discard what the customer sends us and just hash their pass.
    let encrypted_password = hash(password.as_str(), DEFAULT_COST).unwrap();

    // The default pw details, they can override them though...
    let default_pwd = pwdetails::new_pw_details(&email);
    let pwd = lift_pw_params(hashmap, default_pwd);

    // Create a user struct with these details.
    let new_user = users::create_new(email,encrypted_password,pwd);

    // Store/Return it.
    let conn = db::get_connection();
    db::add_user(&conn,&new_user);
    Ok(new_user)
}
fn attempt_get(key: &String, hashmap: &Value) -> Result<Value,String> {
    match hashmap.get(key) {
        None => Err(format!("Missing valid {}", key)),
        Some(json_val) => Ok(json_val.clone())
    }
}
fn lift_pw_params(hashmap: &Value, default_pwd: pwdetails::PasswordDetails) -> pwdetails::PasswordDetails {
    pwdetails::PasswordDetails {
        pw_func:    hashmap.get("pw_func")    .unwrap_or(&json!(default_pwd.pw_func)).as_str().unwrap().to_string(),
        pw_alg:     hashmap.get("pw_alg")     .unwrap_or(&json!(default_pwd.pw_alg)).as_str().unwrap().to_string(),
        pw_cost:    hashmap.get("pw_cost")    .unwrap_or(&json!(default_pwd.pw_cost)).as_u64().unwrap() as i32,
        pw_key_size:hashmap.get("pw_key_size").unwrap_or(&json!(default_pwd.pw_key_size)).as_u64().unwrap() as i32,
        pw_nonce:   hashmap.get("pw_nonce")   .unwrap_or(&json!(default_pwd.pw_nonce)).as_str().unwrap().to_string(),
        pw_salt:    hashmap.get("pw_salt")    .unwrap_or(&json!(default_pwd.pw_salt)).as_str().unwrap().to_string(),
        version:    hashmap.get("version")    .unwrap_or(&json!(default_pwd.version)).as_str().unwrap().to_string(),
    }
}