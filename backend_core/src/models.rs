use schema::{users,items};
use chrono::prelude::*;

#[derive(Queryable,Insertable,Debug,Clone)]
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

#[derive(AsChangeset)]
#[table_name = "users"]
pub struct UserUpdateChange {
    pub updated_at: NaiveDateTime,

    pub pw_func: Option<String>,
    pub pw_alg: Option<String>,
    pub pw_cost: Option<i32>,
    pub pw_key_size: Option<i32>,
    pub pw_nonce: Option<String>,
    pub encrypted_password: Option<String>,
    pub pw_salt: Option<String>,
    pub version: Option<String>,
}
impl Default for UserUpdateChange {
    fn default() -> Self {
        UserUpdateChange {
            updated_at: Utc::now().naive_utc(),
            pw_func: None,
            pw_alg: None,
            pw_cost: None,
            pw_key_size: None,
            pw_nonce: None,
            encrypted_password: None,
            pw_salt: None,
            version: None,
        }
    }
}

#[derive(Queryable,Insertable,Debug,Clone)]
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
    pub last_user_agent: Option<String>,
}
