extern crate hyper;
extern crate hyper_native_tls;
extern crate standardfile;

use standardfile::env;
use standardfile::service;
//use hyper::server::Http;
use hyper::server::Server;
use hyper_native_tls::NativeTlsServer;

// To test https, create a local self-signed cert:
// openssl req -x509 -newkey rsa:4096 -nodes -keyout localhost.key -out localhost.crt -days 3650
// openssl pkcs12 -export -out identity.p12 -inkey localhost.key -in localhost.crt --password mypass
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
    if args.is_present("ssl") {
        let identity = args.value_of("TLSIdentity").unwrap();
        let pass = args.value_of("TLSPassword").unwrap();
        let ssl = NativeTlsServer::new(identity, pass).unwrap();
        let server = Server::https(addr.as_str(), ssl).unwrap();
        let _guard = server.handle(service::handle);
    } else {
        let server = Server::http(addr.as_str()).unwrap();
        let _guard = server.handle(service::handle);
    }
}
