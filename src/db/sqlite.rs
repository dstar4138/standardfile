use diesel;
use diesel::prelude::*;
use schema::{users,items};
use models::{User,Item};
use chrono::NaiveDateTime;

use db::{
    DbConnection,
    StandardFileStorage
};

// Unwrap the generic wrapper
fn get_conn<'a>(db: &'a DbConnection) -> &'a SqliteConnection {
    match db {
        &DbConnection::Sqlite { ref conn } => conn,
        _ => panic!("Unable to get database connection.")
    }
}
impl StandardFileStorage for DbConnection {
    fn add_user(self: &Self, user: &User) -> () {
        diesel::insert_into(users::table)
            .values(user)
            .execute(get_conn(self))
            .expect("Error inserting new user");
    }
    fn update_user(self: &Self, user: User) -> () {
        use schema::users::dsl::*;
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
            .execute(get_conn(self))
            .expect("Error in updating user");
    }
    fn add_or_update_item(self: &Self, item: Item) -> Result<Item, Item> {
        match diesel::replace_into(items::table)
            .values(&item)
            .execute(get_conn(self)) {
            Err(_) => Err(item),
            Ok(_) => Ok(item)
        }
    }

    fn find_user_by_email(self: &Self, user_email: &String) -> Option<User> {
        use schema::users::dsl::{users, email};
        users.filter(email.eq(user_email))
            .limit(1)
            .get_result::<User>(get_conn(self))
            .optional()
            .unwrap()
    }
    fn find_user_by_uuid(self: &Self, user_uuid: &String) -> Option<User> {
        use schema::users::dsl::{users, uuid};
        users.filter(uuid.eq(user_uuid))
            .limit(1)
            .get_result::<User>(get_conn(self))
            .optional()
            .unwrap()
    }

    fn get_items(self: &Self, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(get_conn(self))
            .optional()
            .unwrap()
    }
    fn get_items_older_or_equal_to(self: &Self, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid).and(updated_at.ge(datetime)))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(get_conn(self))
            .optional()
            .unwrap()
    }
    fn get_items_older_than(self: &Self, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
        use schema::items::dsl::{items, user_uuid, updated_at};
        items.filter(user_uuid.eq(users_uuid).and(updated_at.gt(datetime)))
            .limit(limit)
            .order(updated_at)
            .load::<Item>(get_conn(self))
            .optional()
            .unwrap()
    }
}
