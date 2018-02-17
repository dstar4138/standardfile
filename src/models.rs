use schema::{users,items};
use chrono::prelude::*;

#[derive(Queryable,Insertable)]
#[table_name = "users"]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub pw_func: String,
    pub pw_alg: String,
    pub pw_cost: i32,
    pub pw_key_size: i32,
    pub pw_nonce: String,
    pub encrypted_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub pw_salt: String,
    pub version: String,
}

#[derive(Queryable,Insertable)]
#[table_name = "items"]
pub struct Item {
    pub uuid: String,
    pub content: Vec<u8>, // Base64
    pub content_type: String,
    pub enc_item_key: String, // Base64
    pub auth_hash: String, // Hex
    pub user_uuid: String,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_user_agent: String,
}


///
/// Models for exporting data out of the server.
///

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct MinimalItem {
    pub uuid: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub created_at: NaiveDateTime,
}