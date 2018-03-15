#[macro_use]
extern crate diesel;
extern crate chrono;

pub mod models;
pub mod schema;

use chrono::NaiveDateTime;
pub use models::{User,Item};

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