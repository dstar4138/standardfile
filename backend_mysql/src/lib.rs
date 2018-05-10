extern crate actix;
extern crate chrono;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate backend_core;

use std::env;
use actix::prelude::*;
use diesel::prelude::*;
use r2d2::{Pool};
use r2d2_diesel::ConnectionManager;

use backend_core::*;
use backend_core::schema::{self,users};

const DATABASE_HOST: &'static str = "DB_HOST";
const DATABASE_PORT: &'static str = "DB_PORT";
const DATABASE_DATABASE: &'static str = "DB_DATABASE";
const DATABASE_USERNAME: &'static str = "DB_USERNAME";
const DATABASE_PASSWORD: &'static str = "DB_PASSWORD";

fn get_or_panic(key: &str, error: &str) -> String {
    match env::var(key) {
        Ok(val) => val.clone(),
        _       => panic!("No {} given. {}",key,error)
    }
}

fn build_db_uri() -> String {
    let host = get_or_panic(DATABASE_HOST, "Can't locate database host!");
    let port = get_or_panic(DATABASE_PORT, "Can't locate database port!");
    let db_name = get_or_panic(DATABASE_DATABASE, "Can't locate database!");
    let username = get_or_panic(DATABASE_USERNAME, "Can't locate database username!");
    let password = get_or_panic(DATABASE_PASSWORD, "Can't locate database password!");
    format!("mysql://{}:{}@{}:{}/{}", username,password,host,port,db_name)
}

pub struct DBConnection {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl Actor for DBConnection {
    type Context = SyncContext<Self>;
}

impl StandardFileStorage for DBConnection {
    type Manager = ConnectionManager<MysqlConnection>;

    fn new_manager() -> Self::Manager {
        ConnectionManager::<MysqlConnection>::new(build_db_uri())
    }

    fn new(pool : Pool<Self::Manager>) -> DBConnection {
        DBConnection { pool }
    }
}

impl Handler<AddUser> for DBConnection {
    type Result = StandardFileResult<()>;

    fn handle(&mut self, msg: AddUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn : &MysqlConnection = &self.pool.get().expect("Unable to get connection from pool");
        match diesel::insert_into(users::table)
            .values(&msg.user)
            .execute(conn) {
            Err(_) => Err(DBError::QueryError),
            Ok(_) => Ok(())
        }
    }
}

impl Handler<UpdateUser> for DBConnection {
    type Result = StandardFileResult<User>;

    fn handle(&mut self, msg: UpdateUser, _ctx: &mut Self::Context) -> Self::Result {
        use backend_core::schema::users::dsl::*;
        let user = msg.user;
        let conn : &MysqlConnection = &self.pool.get().expect("Unable to get connection from pool");
        match diesel::update(users.filter(uuid.eq(&msg.uuid)))
            .set(&user)
            .get_result::<User>(conn) {
            Err(_) => Err(DBError::QueryError),
            Ok(user) => Ok(user),
        }
    }
}

impl Handler<FindUserByEmail> for DBConnection {
    type Result = StandardFileResult<Option<User>>;

    fn handle(&mut self, msg: FindUserByEmail, _ctx: &mut Self::Context) -> Self::Result {
        use backend_core::schema::users::dsl::{users, email};
        let user_email = msg.email;
        let conn : &MysqlConnection = &self.pool.get().expect("Unable to get connection from pool");
        let res = users.filter(email.eq(user_email))
            .limit(1)
            .get_result::<User>(conn)
            .optional()
            .unwrap();
        Ok(res)
    }
}

impl Handler<FindUserByUUID> for DBConnection {
    type Result = StandardFileResult<Option<User>>;

    fn handle(&mut self, msg: FindUserByUUID, _ctx: &mut Self::Context) -> Self::Result {
        use backend_core::schema::users::dsl::{users, uuid};
        let user_uuid = msg.uuid;
        let conn : &MysqlConnection = &self.pool.get().expect("Unable to get connection from pool");
        let res = users.filter(uuid.eq(user_uuid))
            .limit(1)
            .get_result::<User>(conn)
            .optional()
            .unwrap();
        Ok(res)
    }
}

impl Handler<GetAndUpdateItems> for DBConnection {
    type Result = StandardFileResult<Option<Vec<Item>>>;

    fn handle(&mut self, msg: GetAndUpdateItems, _ctx: &mut Self::Context) -> Self::Result {
        use backend_core::schema::items::dsl::{items, user_uuid, updated_at};
        let users_uuid = msg.user_uuid;
        let limit = msg.limit;
        let conn : &MysqlConnection = &self.pool.get().expect("Unable to get connection from pool");
        for item in msg.items.iter() {
            let _res = diesel::replace_into(schema::items::table)
                .values(item)
                .execute(conn);
        }
        let res = match msg.datetime {
            None =>
                    items.filter(user_uuid.eq(users_uuid))
                         .limit(limit)
                         .order(updated_at)
                         .load::<Item>(conn)
                         .optional()
                         .unwrap(),
            Some(datetime) => if msg.is_inclusive {
                    items.filter(user_uuid.eq(users_uuid).and(updated_at.ge(datetime)))
                         .limit(limit)
                         .order(updated_at)
                         .load::<Item>(conn)
                         .optional()
                         .unwrap()
            } else {
                    items.filter(user_uuid.eq(users_uuid).and(updated_at.gt(datetime)))
                         .limit(limit)
                         .order(updated_at)
                         .load::<Item>(conn)
                         .optional()
                         .unwrap()
            }
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_build_db_uri_without_database_match_format() {
        env::set_var(DATABASE_HOST,"a");
        env::set_var(DATABASE_PORT,"b");
        env::set_var(DATABASE_DATABASE,"c");
        env::set_var(DATABASE_USERNAME,"d");
        env::set_var(DATABASE_PASSWORD,"e");
        let uri = build_db_uri();
        assert_eq!("mysql://d:e@a:b/c", uri);
    }

}