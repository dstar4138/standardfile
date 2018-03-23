#[macro_use(crate_version)]
extern crate clap;

#[macro_use]
extern crate cfg_if;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate gotham_derive;
extern crate gotham;
extern crate futures;
extern crate hyper;
extern crate mime;

extern crate serde;
extern crate base64;
extern crate bcrypt;
extern crate chrono;
extern crate crypto;
extern crate dotenv;
extern crate uuid;
extern crate env_logger;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;

extern crate backend_core;
cfg_if! {
    if #[cfg(feature = "mysql")] {
        extern crate backend_mysql;
    } else { // We default to using sqlite.
        extern crate backend_sqlite;
    }
}

pub mod db;
pub mod env;
pub mod items;
pub mod service;
pub mod users;
pub mod pwdetails;
pub mod tokens;
pub mod util;
pub mod api;
