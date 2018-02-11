extern crate iron;
extern crate standardfile;

use iron::prelude::*;
use standardfile::env;
use standardfile::service;

fn main() {
    let args = env::setup_env_arg_parser().get_matches();
    let port = args.value_of(env::ARG_PORT).unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    let server = Iron::new(service::handler());
    match server.http(addr.as_str()) {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
