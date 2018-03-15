use std::env;
use env_logger;
use dotenv::dotenv;
use clap::{App, Arg};

/* Global App Values. */
pub const APP_NAME: &'static str = "standardfiled";
pub const LOCALHOST: &'static str = "0.0.0.0";

/* Default Parameters. */
pub const DEFAULT_PORT: &'static str = "8080";

/* Environment variables */
const SECRET_KEY_BASE: &'static str = "SECRET_KEY_BASE";
const SALT_PSEUDO_NONCE: &'static str = "SALT_PSEUDO_NONCE";

/* Argument Names */
pub const ARG_PORT: &'static str = "port";

pub fn setup_env_arg_parser<'a,'b>() -> App<'a,'b> {
    env_logger::init();
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

pub fn get_secret_key() -> String {
    get_or_panic(SECRET_KEY_BASE, "Will fail to locally encrypt!")
}

pub fn get_pseudo_salt() -> String {
    get_or_panic(SALT_PSEUDO_NONCE, "Will fail to securely hash!")
}