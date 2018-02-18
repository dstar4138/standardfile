use std::env;
use dotenv::dotenv;
use clap::{App, Arg};

/* Global App Values. */
pub const APP_NAME: &'static str = "standardfiled";
pub const LOCALHOST: &'static str = "0.0.0.0";

/* Default Parameters. */
pub const DEFAULT_PORT: &'static str = "8080";

/* Environment variables */
const DATABASE_PATH: &'static str = "DB_PATH"; // if sqlite.
const DATABASE_HOST: &'static str = "DB_HOST";
const DATABASE_PORT: &'static str = "DB_PORT";
const DATABASE_DATABASE: &'static str = "DB_DATABASE";
const DATABASE_USERNAME: &'static str = "DB_USERNAME";
const DATABASE_PASSWORD: &'static str = "DB_PASSWORD";
const SECRET_KEY_BASE: &'static str = "SECRET_KEY_BASE";
const SALT_PSEUDO_NONCE: &'static str = "SALT_PSEUDO_NONCE";

/* Argument Names */
pub const ARG_PORT: &'static str = "port";

pub fn setup_env_arg_parser<'a,'b>() -> App<'a,'b> {
    dotenv().ok();

    App::new(APP_NAME)
        .version(crate_version!())
        .arg(
            Arg::with_name(ARG_PORT)
                .short("p")
                .long(ARG_PORT)
                .value_name("PORT")
                .default_value(DEFAULT_PORT))
}

fn get_or_panic(key: &str, error: &str) -> String {
    match env::var(key) {
        Ok(val) => val.clone(),
        _       => panic!("No {} given. {}",key,error)
    }
}

pub fn get_database_hostport() -> String {
    let host = get_or_panic(DATABASE_HOST, "Can't locate database host!");
    let port = get_or_panic(DATABASE_PORT, "Can't locate database port!");
    format!("{}:{}",host,port)
}
pub fn get_database_path() -> String {
    get_or_panic(DATABASE_PATH, "Can't locate database file!")
}
pub fn get_database_name() -> String {
    get_or_panic(DATABASE_DATABASE, "Can't locate database!")
}
pub fn get_database_creds() -> (String,String) {
    let username = get_or_panic(DATABASE_USERNAME, "Can't locate database username!");
    let password = get_or_panic(DATABASE_PASSWORD, "Can't locate database password!");
    (username,password)
}

pub fn get_secret_key() -> String {
    get_or_panic(SECRET_KEY_BASE, "Will fail to locally encrypt!")
}

pub fn get_pseudo_salt() -> String {
    get_or_panic(SALT_PSEUDO_NONCE, "Will fail to securely hash!")
}