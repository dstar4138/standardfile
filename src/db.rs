use env;
use diesel;
use diesel::prelude::*;
use schema::{users,items};
use models::{User,Item};

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

pub fn update_user(conn: &SqliteConnection, user: User) -> () {
    use schema::users::dsl::*;
    diesel::update(users.filter(email.eq(&user.email)))
        .set((
                encrypted_password.eq(&user.encrypted_password),
                updated_at.eq(&user.updated_at)
            ))
        .execute(conn)
        .expect("Error updating existing user");
}

pub fn add_item(conn: &SqliteConnection, item: Item) -> () {
    diesel::insert_into(items::table)
        .values(&item)
        .execute(conn)
        .expect("Error inserting new item");
}

pub fn update_item(conn: &SqliteConnection, item: Item) -> () {
    use schema::items::dsl::*;
    diesel::update(items.filter(uuid.eq(&item.uuid)))
        .set((
            content.eq(&item.content),
            content_type.eq(&item.content_type),
            enc_item_key.eq(&item.enc_item_key),
            auth_hash.eq(&item.auth_hash),
            updated_at.eq(&item.updated_at),
            deleted.eq(&item.deleted),
            last_user_agent.eq(&item.last_user_agent)
        ))
        .execute(conn)
        .expect("Error updating existing item");
}

pub fn find_user_by_email(conn: &SqliteConnection, user_email: &String) -> Option<User> {
    use schema::users::dsl::{users,email};
    users.filter(email.eq(user_email))
        .limit(1)
        .get_result::<User>(conn)
        .optional()
        .unwrap()
}

pub fn get_items_by_user_uuid<'a,T>(conn: &SqliteConnection, users_uuid: &String) -> Option<Vec<Item>> {
    use schema::items::dsl::{items,user_uuid};
    items.filter(user_uuid.eq(users_uuid))
        .load::<Item>(conn)
        .optional()
        .unwrap()
}
