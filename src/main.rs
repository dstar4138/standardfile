extern crate standardfile;
extern crate gotham;

use standardfile::env;
use standardfile::service;

fn main() {
    let args = env::setup_env_arg_parser().get_matches();
    let port = args.value_of(env::ARG_PORT).unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    println!("Starting at: {:?}",addr);
    gotham::start(addr,service::router());
}
