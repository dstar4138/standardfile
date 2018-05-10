extern crate standardfile;
extern crate actix_web;
extern crate actix;
extern crate r2d2;

use actix::prelude::*;
use actix_web::{server};

use standardfile::api;
use standardfile::env;
use standardfile::service;
use standardfile::db::{self, StandardFileStorage};

fn main() {
    let args = env::setup_env_arg_parser().get_matches();
    let port = args.value_of(env::ARG_PORT).unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    // Create pool around the db manager
    let manager = db::StandardFileStorageDB::new_manager();
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    // Place it on the heap and give it an address
    let sys = actix::System::new(env::APP_NAME);
    let arbiter = SyncArbiter::start(db::POOL_SIZE, move || {
        db::StandardFileStorageDB::new( pool.clone() )
    });

    // Start Http Server
    println!("Started at: {:?}", addr);
    server::HttpServer::new(move || {
        service::app(api::ServiceState{ db: arbiter.clone() })
    })
        .bind(addr).expect("Unable to bind to port!")
        .start();

    let _ = sys.run();
}
