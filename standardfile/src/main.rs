extern crate standardfile;
extern crate actix_web;

use actix_web::{server};

use standardfile::env;
use standardfile::service;
use standardfile::db;

fn main() {
    let args = env::setup_env_arg_parser().get_matches();
    let port = args.value_of(env::ARG_PORT).unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    println!("Testing DB connection");
    let _db = db::get_connection().expect("Failed to get connection!");

    println!("Starting at: {:?}",addr);
    server::HttpServer::new(service::app)
        .bind(addr).expect("Unable to bind to port!")
        .run()
}
