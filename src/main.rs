extern crate hyper;
extern crate standardfile;

use standardfile::env;
use standardfile::service;
//use hyper::server::Http;
use hyper::server::Server;

fn main() {
    let args = env::build_arg_parser().get_matches();
    let port = args.value_of("port").unwrap();
    let addr: String = format!("{}:{}", env::LOCALHOST, port).parse().unwrap();

    /* POST hyper version 0.10
    let server = Http::new()
        .bind(&addr, || Ok(service::StandardFileService))
        .unwrap();

    server.run().unwrap();
    */
    println!("listening on {}",addr);
    let server = Server::http(addr.as_str()).unwrap();
    let _guard = server.handle(service::handle);
}
