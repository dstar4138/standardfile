use env;
use diesel;
use diesel::prelude::*;
use schema::{users,items};
use models::{User,Item};
use chrono::NaiveDateTime;

// TODO: find a way to abstract the types for easy plug-and-play.
pub fn get_connection() -> SqliteConnection
{
//    if cfg!(mysql) {
//        let (user,password) = env::get_database_creds();
//        let host = env::get_database_hostport();
//        let database_name = env::get_database_name();
//        let connection_url = format!("mysql://{}:{}/{}/{}",
//                                     user,password,host,database_name);
//        create_connection(connection_url.as_str())
//    } else {
        //create_connection(":memory")
        let path = env::get_database_path();
        create_connection(path.as_str())
//    }
}
fn create_connection(connection_url: &str) -> SqliteConnection {
//    if cfg!(mysql) {
//        diesel::MysqlConnection::establish(connection_url)
//            .expect(&format!("Error connection to {}", connection_url))
//    } else {
        diesel::SqliteConnection::establish(connection_url)
            .expect(&format!("Error connecting to {}", connection_url))
//    }
}

pub fn add_user(conn: &SqliteConnection, user: &User) -> () {
    diesel::insert_into(users::table)
        .values(user)
        .execute(conn)
        .expect("Error inserting new user");
}

pub fn update_user(conn: &SqliteConnection, user: User) -> Result<User,()> {
    use schema::users::dsl::*;
    match diesel::update(users.filter(email.eq(&user.email)))
        .set((
                encrypted_password.eq(&user.encrypted_password),
                updated_at        .eq(&user.updated_at),
                pw_func           .eq(&user.pw_func),
                pw_alg            .eq(&user.pw_alg),
                pw_cost           .eq(&user.pw_cost),
                pw_key_size       .eq(&user.pw_key_size),
                pw_nonce          .eq(&user.pw_nonce),
                pw_salt           .eq(&user.pw_salt),
                version           .eq(&user.version)
            ))
        .execute(conn) {
        Err(_) => Err(()),
        Ok(_)  => Ok(user)
    }
}

pub fn add_or_update_item(conn: &SqliteConnection, item: Item) -> Result<Item,Item> {
    match diesel::replace_into(items::table)
        .values(&item)
        .execute(conn) {
        Err(_) => Err(item),
        Ok(_)  => Ok(item)
    }
}

pub fn find_user_by_email(conn: &SqliteConnection, user_email: &String) -> Option<User> {
    use schema::users::dsl::{users,email};
    users.filter(email.eq(user_email))
        .limit(1)
        .get_result::<User>(conn)
        .optional()
        .unwrap()
}
pub fn find_user_by_uuid(conn: &SqliteConnection, user_uuid: &String) -> Option<User> {
    use schema::users::dsl::{users,uuid};
    users.filter(uuid.eq(user_uuid))
        .limit(1)
        .get_result::<User>(conn)
        .optional()
        .unwrap()
}

pub fn find_user_pw_hash_by_uuid(conn: &SqliteConnection, user_uuid: &String) -> Option<String> {
    use schema::users::dsl::{users,uuid,encrypted_password};
    users.filter(uuid.eq(user_uuid))
        .limit(1)
        .select(encrypted_password)
        .get_result::<String>(conn)
        .optional()
        .unwrap()
}
pub fn get_items(conn: &SqliteConnection, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
    use schema::items::dsl::{items,user_uuid,updated_at};
    items.filter(user_uuid.eq(users_uuid))
        .limit(limit)
        .order(updated_at)
        .load::<Item>(conn)
        .optional()
        .unwrap()
}
pub fn get_items_older_or_equal_to(conn: &SqliteConnection, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
    use schema::items::dsl::{items,user_uuid,updated_at};
    items.filter(user_uuid.eq(users_uuid).and(updated_at.ge(datetime)))
        .limit(limit)
        .order(updated_at)
        .load::<Item>(conn)
        .optional()
        .unwrap()
}
pub fn get_items_older_than(conn: &SqliteConnection, datetime: &NaiveDateTime, users_uuid: &String, limit: i64) -> Option<Vec<Item>> {
    use schema::items::dsl::{items,user_uuid,updated_at};
    items.filter(user_uuid.eq(users_uuid).and(updated_at.gt(datetime)))
        .limit(limit)
        .order(updated_at)
        .load::<Item>(conn)
        .optional()
        .unwrap()
}
