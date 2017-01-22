use clap::{App, Arg};

/* Global App Values. */
pub const APP_NAME: &'static str = "standardfiled";
pub const LOCALHOST: &'static str = "0.0.0.0";

/* Default Parameters. */
pub const DEFAULT_PORT: &'static str = "8080";

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

