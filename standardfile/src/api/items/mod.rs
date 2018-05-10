mod sync;
//mod sync_transaction;
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
pub struct SyncRequest {
    sync_token: Option<PaginationToken>,
    cursor_token: Option<PaginationToken>,
    items: Vec<MinimalItem>,

    #[serde(default)]
    limit: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct SyncResponse {
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

use backend_core::models::Item;
impl<'a> From<&'a Item> for MinimalItem {
    fn from(item: &'a Item) -> Self {
        MinimalItem {
            uuid: item.uuid.clone(),
            content: String::from_utf8(item.content.clone()).unwrap(),
            content_type: item.content_type.clone(),
            enc_item_key: item.enc_item_key.clone(),
            auth_hash: Some(item.auth_hash.clone()),
            deleted: item.deleted.clone(),
            created_at: ZuluTimestamp::from_datetime(item.created_at.clone()),
            updated_at: ZuluTimestamp::from_datetime(item.updated_at.clone()),
        }
    }
}

fn minify_items(optional_items: Option<Vec<Item>>) -> Vec<MinimalItem> {
    match optional_items {
        None => vec![],
        Some(items) =>
            items.iter().map(|&ref item: &Item| MinimalItem::from(item)).collect()
    }
}