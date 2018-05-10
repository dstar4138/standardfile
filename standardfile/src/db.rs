use actix::prelude::*;
pub use backend_core::*;
pub const POOL_SIZE: usize = 5;
cfg_if! {
    if #[cfg(feature = "mysql")] {
        use backend_mysql::*;
    } else { // We default to using sqlite.
        use backend_sqlite::*;
    }
}

pub type StandardFileStorageDB = DBConnection; // assumes creation and import from backend.
pub type DBAddr = Addr<Syn, StandardFileStorageDB>;
