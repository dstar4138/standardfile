use std::env;
use clap::{App, Arg};

/* Global App Values. */
pub const APP_NAME: &'static str = "standardfiled";
pub const LOCALHOST: &'static str = "0.0.0.0";

/* Default Parameters. */
pub const DEFAULT_PORT: &'static str = "8080";

/* Environment variables */
const DATABASE_PATH: &'static str = "DB_PATH";
const SALT_PSEUDO_NONCE: &'static str = "SALT_PSEUDO_NONCE";

//TODO:Allow to pull from ENV or file so pass isn't in args.
pub fn build_arg_parser<'a,'b>() -> App<'a,'b> {
    App::new(APP_NAME)
        .version(crate_version!())
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .default_value(DEFAULT_PORT))

        .arg(
            // Toggle on SSL.
            Arg::with_name("ssl")
                .long("ssl")
                .index(1)
                .requires_all(&["TLSIdentity","TLSPassword"]))
        .arg(
            Arg::with_name("TLSIdentity")
                .requires("ssl")
                .short("I")
                .long("TLSIdentityFile")
                .value_name("FILE"))
        .arg(
            Arg::with_name("TLSPassword")
                .requires("ssl")
                .short("P")
                .long("TLSPassword")
                .value_name("PASS"))
}

pub fn get_database_path() -> String {
    match env::var(DATABASE_PATH) {
        Ok(val) => val.clone(),
        _       => panic!("No DB_PATH given. Can't locate database!")
    }
}

pub fn get_pseudo_salt() -> String {
    match env::var(SALT_PSEUDO_NONCE) {
        Ok(val) => val.clone(),
        _       => panic!("No SALT_PSEUDO_NONCE given. Will fail to securely hash!")
    }
}
