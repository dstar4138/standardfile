#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate actix;
extern crate r2d2;

pub mod models;
pub mod schema;

use actix::prelude::*;
use r2d2::{Pool, ManageConnection};
use chrono::NaiveDateTime;
pub use models::{User,UserUpdateChange,Item};

pub struct AddUser { pub user: User }
pub struct UpdateUser { pub uuid: String, pub user: UserUpdateChange }

pub struct FindUserByEmail { pub email: String }
pub struct FindUserByUUID { pub uuid: String }

// Todo: Figure out how to split these into two distinct clean calls.
pub struct GetAndUpdateItems {
    // Items to save.
    pub items: Vec<Item>,

    // How to get items:
    pub user_uuid: String,
    pub limit: i64,
    pub datetime: Option<NaiveDateTime>,
    pub is_inclusive: bool,
}

#[derive(Debug,Clone)]
pub enum DBError {
    ConnectionError,
    QueryError,
}

pub type StandardFileResult<T> = Result<T,DBError>;

impl Message for AddUser {
    type Result = StandardFileResult<()>;
}
impl Message for UpdateUser {
    type Result = StandardFileResult<User>;
}

impl Message for FindUserByEmail {
    type Result = StandardFileResult<Option<User>>;
}
impl Message for FindUserByUUID {
    type Result = StandardFileResult<Option<User>>;
}

impl Message for GetAndUpdateItems {
    type Result = StandardFileResult<Option<Vec<Item>>>;
}

pub trait StandardFileStorage where Self :
    Handler<AddUser> +
    Handler<UpdateUser> +
    Handler<FindUserByEmail> +
    Handler<FindUserByUUID> +
    Handler<GetAndUpdateItems> +
    Actor
{
    type Manager: ManageConnection + Sized + 'static;

    fn new_manager() -> Self::Manager;
    fn new(pool: Pool<Self::Manager>) -> Self;
}

//#[cfg(test)]
mod test {
    use super::*;
    use r2d2::{Pool, ManageConnection, Error};
    use models::{User, Item};

    pub struct TestManageConnection;

    impl ManageConnection for TestManageConnection {
        type Connection = ();
        type Error = Error;

        fn connect(&self) -> Result<Self::Connection, Self::Error> {
            Ok(())
        }
        fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
            Ok(())
        }
        fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
            false
        }
    }

    pub struct TestStandardFileStorage {
        // All types are wrapped in Option so that on None we can panic.
        pub add_user:               Option<StandardFileResult<()>>,
        pub update_user:            Option<StandardFileResult<User>>,
        pub find_user_by_email:     Option<StandardFileResult<Option<User>>>,
        pub find_user_by_uuid:      Option<StandardFileResult<Option<User>>>,
        pub get_and_update_items:   Option<StandardFileResult<Option<Vec<Item>>>>,
    }

    impl Default for TestStandardFileStorage {
        fn default() -> Self {
            TestStandardFileStorage {
                add_user: None,
                update_user: None,
                find_user_by_email: None,
                find_user_by_uuid: None,
                get_and_update_items: None,
            }
        }
    }

    fn ret_fake<T: Clone>(res: &Option<T>, name: &'static str) ->  T {
        match res {
            &None => panic!("No result given to Test Storage for {}!", name),
            &Some(ref x) => x.clone()
        }
    }

    impl Actor for TestStandardFileStorage {
        type Context = SyncContext<Self>;
    }
    impl Handler<AddUser> for TestStandardFileStorage {
        type Result = StandardFileResult<()>;
        fn handle(&mut self, _msg: AddUser, _ctx: &mut Self::Context) -> Self::Result {
            ret_fake(&self.add_user, "AddUser")
        }
    }
    impl Handler<UpdateUser> for TestStandardFileStorage {
        type Result = StandardFileResult<User>;
        fn handle(&mut self, _msg: UpdateUser, _ctx: &mut Self::Context) -> Self::Result {
            ret_fake(&self.update_user, "UpdateUser")
        }
    }
    impl Handler<FindUserByEmail> for TestStandardFileStorage {
        type Result = StandardFileResult<Option<User>>;
        fn handle(&mut self, _msg: FindUserByEmail, _ctx: &mut Self::Context) -> Self::Result {
            ret_fake(&self.find_user_by_email, "FindUserByEmail")
        }
    }
    impl Handler<FindUserByUUID> for TestStandardFileStorage {
        type Result = StandardFileResult<Option<User>>;
        fn handle(&mut self, _msg: FindUserByUUID, _ctx: &mut Self::Context) -> Self::Result {
            ret_fake(&self.find_user_by_uuid, "FindUserByUUID")
        }
    }
    impl Handler<GetAndUpdateItems> for TestStandardFileStorage {
        type Result = StandardFileResult<Option<Vec<Item>>>;
        fn handle(&mut self, _msg: GetAndUpdateItems, _ctx: &mut Self::Context) -> Self::Result {
            ret_fake(&self.get_and_update_items, "GetAndUpdateItems")
        }
    }

    impl StandardFileStorage for TestStandardFileStorage {
        type Manager = TestManageConnection;

        fn new_manager() -> Self::Manager {
            TestManageConnection
        }

        fn new(_pool: Pool<Self::Manager>) -> Self {
            TestStandardFileStorage::default()
        }
    }
}
