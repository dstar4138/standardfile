mod sync;
mod timestamp;
mod pagination;

pub use self::sync::sync;
pub use self::pagination::{PaginationToken};
pub use self::pagination::serde::*;
pub use self::timestamp::ZuluTimestamp;


// TODO: can this be a From/Into so it can do casting for us.
use chrono::NaiveDateTime;
pub trait IsDateTime {
    fn to_datetime(&self) -> NaiveDateTime;
    fn from_datetime(datetime: NaiveDateTime) -> Self;
}

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    retrieved_items: Vec<MinimalItem>,
    saved_items: Vec<MinimalItem>,
    unsaved: Vec<MinimalItem>,
    sync_token: PaginationToken,
    cursor_token: Option<PaginationToken>,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct MinimalItem {
    pub uuid: String,
    pub content: String,
    pub content_type: String,
    pub enc_item_key: String,
    pub auth_hash: Option<String>,
    #[serde(default)]
    pub deleted: bool,
    pub created_at: ZuluTimestamp,
    #[serde(default)] // Will deserialize into current_time if none given; i.e. item.touch
    pub updated_at: ZuluTimestamp,
}