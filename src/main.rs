extern crate iron;
extern crate hyper_native_tls;
extern crate standardfile;

use iron::prelude::*;
use standardfile::env;
use standardfile::service;
use hyper_native_tls::NativeTlsServer;

fn main() {
    let args = env::build_arg_parser().get_matches();
    let port = args.value_of("port").unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    let server = Iron::new(service::handler());
    let res = 
        if args.is_present("ssl") {
            let identity = args.value_of("TLSIdentity").unwrap();
            let pass = args.value_of("TLSPassword").unwrap();
            let ssl = NativeTlsServer::new(identity, pass).unwrap();
            server.https(addr.as_str(), ssl)
        } else {
            server.http(addr.as_str())
        };

    match res {
        Result::Ok(listening) => println!("{:?}", listening),
        Result::Err(err) => panic!("{:?}", err),
    }
}
