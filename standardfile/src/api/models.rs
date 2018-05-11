use api::timestamp::ZuluTimestamp;
use db::models::{User,Item};
use tokens;

#[derive(Serialize, Deserialize)]
struct MinimalUser {
    uuid: String,
    email: String,
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

#[derive(Serialize, Deserialize)]
pub struct JwtMsg {
    user: MinimalUser,
    token: String,
}

impl<'a> From<&'a User> for JwtMsg {
    fn from(user: &'a User) -> JwtMsg {
        JwtMsg {
            user: MinimalUser {
                uuid: user.uuid.clone(),
                email: user.email.clone(),
            },
            token: tokens::user_to_jwt(&user).unwrap(),
        }
    }
}

impl<'a> From<&'a Item> for MinimalItem {
    fn from(item: &'a Item) -> Self {
        MinimalItem {
            uuid: item.uuid.clone(),
            content: String::from_utf8(item.content.clone()).unwrap(),
            content_type: item.content_type.clone(),
            enc_item_key: item.enc_item_key.clone(),
            auth_hash: Some(item.auth_hash.clone()),
            deleted: item.deleted.clone(),
            created_at: ZuluTimestamp::from(&item.created_at),
            updated_at: ZuluTimestamp::from(&item.updated_at),
        }
    }
}

pub fn minify_items(optional_items: Option<Vec<Item>>) -> Vec<MinimalItem> {
    match optional_items {
        None => vec![],
        Some(items) =>
            items.iter().map(|&ref item: &Item| MinimalItem::from(item)).collect()
    }
}