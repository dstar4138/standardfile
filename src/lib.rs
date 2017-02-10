#![feature(custom_derive)]

#[macro_use(crate_version)]
extern crate clap;

#[macro_use(router)]
extern crate router;

#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate iron;
extern crate uuid;
extern crate rusqlite;
extern crate rustc_serialize;
extern crate serde_json;
extern crate urlencoded;
extern crate crypto;

pub mod db;
pub mod env;
pub mod items;
pub mod service;
pub mod users;

pub mod auth;
