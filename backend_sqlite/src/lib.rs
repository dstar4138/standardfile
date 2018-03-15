extern crate chrono;
extern crate diesel;
extern crate backend_core;

use std::env;
use diesel::prelude::*;
use backend_core::schema::{users,items};
use backend_core::{User,Item,StandardFileStorage};
use chrono::NaiveDateTime;

const DATABASE_PATH: &'static str = "DB_PATH";

fn get_or_panic(key: &str, error: &str) -> String {
    match env::var(key) {
        Ok(val) => val.clone(),
        _       => panic!("No {} given. {}",key,error)
    }
}

pub fn get_connection() -> Result<Box<StandardFileStorage>,ConnectionError> {
    let path = get_or_panic(DATABASE_PATH, "Can't locate database file!");
    let conn = diesel::SqliteConnection::establish(&path)?;
    Ok(Box::new(DbConnection { conn }))
}

struct DbConnection {
    conn: SqliteConnection,
}

impl StandardFileStorage for DbConnection {
    fn add_user(self: &Self, user: &User) -> () {
        diesel::insert_into(users::table)
            .values(user)
            .execute(&self.conn)
            .expect("Error inserting new user");
    }
    fn update_user(self: &Self, user: User) -> () {
        use backend_core::schema::users::dsl::*;
        diesel::update(users.filter(email.eq(&user.email)))
            .set((
                encrypted_password.eq(&user.encrypted_password),
                updated_at.eq(&user.updated_at),
                pw_func.eq(&user.pw_func),
                pw_alg.eq(&user.pw_alg),
                pw_cost.eq(&user.pw_cost),
                pw_key_size.eq(&user.pw_key_size),
                pw_nonce.eq(&user.pw_nonce),
                pw_salt.eq(&user.pw_salt),
                version.eq(&user.version)
            ))
            .execute(&self.conn)
            .expect("Error in updating user");
    }
    fn add_or_update_item(self: &Self, item: Item) -> Result<Item, Item> {
        match diesel::replace_into(items::table)
            .values(&item)
            .execute(&self.conn) {
            Err(_) => Err(item),
            Ok(_) => Ok(item)
        }
    }

    fn find_user_by_email(self: &Self, user_email: &String) -> Option<User> {
        use backend_core::schema::users::dsl::{users, email};
        users.filter(email.eq(user_email))
            .limit(1)
            .get_result::<User>(&self.conn)
            .optional()
            .unwrap()
    }
    fn find_user_by_uuid(self: &Self, user_uuid: &String) -> Option<User> {
        use backend_core::schema::users::dsl::{users, uuid};
        users.filter(uuid.eq(user_uuid))
            .limit(1)
            .get_result::<User>(&self.conn)
            .optional()
            .unwrap()
    }

    fn get_items(self: &Self, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use backend_core::schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(&self.conn)
            .optional()
            .unwrap()
    }
    fn get_items_older_than(self: &Self, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use backend_core::schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid).and(updated_at.gt(datetime)))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(&self.conn)
            .optional()
            .unwrap()
    }
    fn get_items_older_or_equal_to(self: &Self, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use backend_core::schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid).and(updated_at.ge(datetime)))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(&self.conn)
            .optional()
            .unwrap()
    }
}