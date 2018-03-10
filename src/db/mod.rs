mod sqlite;
pub use self::sqlite::*;

use env;
use diesel;
use diesel::prelude::*;

use models::{User, Item};
use chrono::NaiveDateTime;

pub enum DbConnection {
  Sqlite { conn: SqliteConnection }
}

//TODO: override with selected database type.
pub fn get_connection() -> Result<DbConnection, ConnectionError> {
    let path = env::get_database_path();
    let conn = diesel::SqliteConnection::establish(&path)?;
    Ok( DbConnection::Sqlite {conn} )
}

pub trait StandardFileStorage {
    fn add_user(&self, user: &User) -> ();
    fn update_user(&self, new_user: User) -> ();
    fn add_or_update_item(&self, item: Item) -> Result<Item,Item>;

    fn find_user_by_email(&self, user_email: &String) -> Option<User>;
    fn find_user_by_uuid(&self, user_uuid: &String) -> Option<User>;

    fn get_items(&self, user_uuid: &String, limit: i64) -> Option<Vec<Item>>;
    fn get_items_older_than(&self, datetime: &NaiveDateTime, user_uuid: &String, limit: i64) -> Option<Vec<Item>>;
    fn get_items_older_or_equal_to(&self, datetime: &NaiveDateTime, user_uuid: &String, limit: i64) -> Option<Vec<Item>>;
}
