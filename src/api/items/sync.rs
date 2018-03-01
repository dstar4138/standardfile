use base64;
use iron::status;
use iron::prelude::*;
use chrono::{NaiveDateTime,Duration};
use serde_json;
use serde_json::value::Value;

use db;
use diesel::prelude::SqliteConnection;
use models::Item;

use api::{
//    encode_error_msg,
    load_json_req_body,
    get_current_user_uuid,
    get_user_agent
};

use util::current_time;

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    retrieved_items: Vec<MinimalItem>,
    saved_items: Vec<MinimalItem>,
    unsaved: Vec<MinimalItem>,
    sync_token: String,
    cursor_token: Option<String>,
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
pub struct MinimalItem {
    pub uuid: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub enc_item_key: String,
    pub auth_hash: String,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn sync(req: &mut Request) -> IronResult<Response> {
    let user_uuid = match get_current_user_uuid(req) {
        Ok(val) => val,
        Err(e) => return Ok(Response::with(e))
    };
    let items = get_sync_items(req);
    let sync_params = get_sync_params(req);
    let conn = db::get_connection();
    let (retrieved_items, cursor_token) = do_sync_get(&user_uuid, sync_params, &conn);
    let user_agent = get_user_agent(req);
    let (saved_items, unsaved) = do_sync_save(&user_uuid, items, &user_agent, &conn);

    // add 1 microsecond to avoid returning same object in subsequent sync, same as ruby code
    let last_updated = current_time() + Duration::microseconds(1);
    let sync_token = generate_sync_token(&last_updated);

    let response = SyncResponse {
        retrieved_items,
        saved_items,
        unsaved,
        sync_token,
        cursor_token
    };

    let res = (status::Ok, serde_json::to_string(&response).unwrap());
    Ok(Response::with(res))
}
fn get_sync_items(req: &mut Request) -> Vec<MinimalItem> {
    match load_json_req_body(req) {
        Err(_) => vec![],
        Ok(ref hashmap) => {
            match hashmap.get("items") {
                None => vec![],
                Some(in_items) =>
                    in_items.as_array().unwrap().iter().map(
                        |&ref val: &Value| {
                            serde_json::from_value(val.to_owned()).unwrap()
                        }
                    ).collect()
            }
        }
    }
}
/// in_sync_token, in_cursor_token, in_limit
fn get_sync_params(req: &mut Request) -> (Option<String>,Option<String>,Option<u32>) {
    match load_json_req_body(req) {
        Err(_) => (None,None,None),
        Ok(ref hashmap) => {
            let in_sync_token = unwrap_decode(hashmap.get("sync_token"));
            let in_cursor_token = unwrap_decode(hashmap.get("cursor_token"));
            match hashmap.get("limit") {
                None => (in_sync_token, in_cursor_token, None),
                Some(v) => {
                    let limit = v.as_u64().unwrap() as u32;
                    (in_sync_token, in_cursor_token, Some(limit))
                }
            }
        }
    }
}
fn unwrap_decode(val: Option<&Value>) -> Option<String> {
    match val {
        None => None,
        Some(v) => Some(v.to_string())
    }
}
fn generate_sync_token(last_update: &NaiveDateTime) -> String {
    let version : u32 = 2;
    let timestamp = last_update.format("%s%f");
    base64::encode(&format!("{}:{}",version,timestamp))
}
fn get_last_update_from_sync_token(sync_token:String) -> NaiveDateTime {
    let bytes = base64::decode(&sync_token).unwrap();
    let token = String::from_utf8(bytes).unwrap();
    let vec : Vec<&str> = token.split(":").collect();
    let version = vec.get(0).unwrap().parse::<u32>().unwrap();
    let timestamp = vec.get(1).unwrap();
    match version {
        1 => NaiveDateTime::parse_from_str(timestamp, "%s").unwrap(),
        _ => NaiveDateTime::parse_from_str(timestamp,"%s%f").unwrap()
    }
}

fn do_sync_get(user_uuid:&String, sync_params: (Option<String>,Option<String>,Option<u32>), conn: &SqliteConnection) -> (Vec<MinimalItem>,Option<String>) {
    let (in_sync_token, in_cursor_token, in_limit) = sync_params;
    let limit = match in_limit {
        None => 100_000,
        Some(val) => val
    };
    let items :Vec<MinimalItem> = match (in_cursor_token, in_sync_token) {
        (None,Some(sync_token)) => {
            let datetime = get_last_update_from_sync_token(sync_token);
            let items = db::get_items_older_or_equal_to(conn, &datetime, user_uuid, limit);
            minify_items(items)
        },
        (Some(cursor_token), _) => {
            let datetime = get_last_update_from_sync_token(cursor_token);
            let items = db::get_items_older_than(conn, &datetime, user_uuid, limit);
            minify_items(items)
        },
        (None, None) => {
            let items = db::get_items(conn, user_uuid, limit);
            minify_items(items)
        }
    };
    let cursor_token = match items.last() {
        None => None,
        Some(last) =>
            Some( generate_sync_token(&last.updated_at) )
    };
    (items,cursor_token)
}
fn minify_items(optional_items: Option<Vec<Item>>) -> Vec<MinimalItem> {
    match optional_items {
        None => vec![],
        Some(items) =>
            items.iter().map(|&ref item: &Item| minify_item(item)).collect()
    }
}
fn minify_item(item: &Item) -> MinimalItem {
    // TODO: Really? do I have to clone all of this?
    MinimalItem {
        uuid: item.uuid.clone(),
        content: item.content.clone(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash: item.auth_hash.clone(),
        deleted: item.deleted.clone(),
        created_at: item.created_at.clone(),
        updated_at: item.updated_at.clone(),
    }
}
fn maximize_item(user_uuid: &String, last_user_agent: &Option<String>, item: &MinimalItem) -> Item {
    Item { //TODO: this looks dumb... is this seriously what i have to do?
        uuid: item.uuid.clone(),
        content: item.content.clone(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash: item.auth_hash.clone(),
        user_uuid: user_uuid.to_owned(),
        deleted: item.deleted.clone(),
        created_at: item.created_at.clone(),
        updated_at: item.updated_at.clone(),
        last_user_agent: last_user_agent.to_owned(),
    }
}
fn update_for_deleted(item: Item) -> Item {
    match item.deleted {
        false => item,
        true  => Item {
            content: vec![],
            enc_item_key: "".to_string(),
            auth_hash: "".to_string(),
            ..item
        }
    }
}

fn unwrap(val : Vec<Result<Item,Item>>) -> Vec<MinimalItem> {
    val.iter().map(
        |res: &Result<Item,Item>|
            match res {
                &Ok(ref item) => item,
                &Err(ref item) => item
        })
        .map(|&ref item: &Item | minify_item(item))
        .collect()
}
fn do_sync_save(user_uuid:&String, items: Vec<MinimalItem>, user_agent: &Option<String>, conn: &SqliteConnection) -> (Vec<MinimalItem>, Vec<MinimalItem>) {
    let (saved_items, unsaved_items) = items
        .iter()
        .map(|&ref item: &MinimalItem| maximize_item(user_uuid, user_agent,item))
        .map(|item: Item| update_for_deleted(item))
        .map(|item: Item| db::add_or_update_item(conn,item))
        .partition(|db_result: &Result<Item,Item>| db_result.is_ok());
    (unwrap(saved_items), unwrap(unsaved_items))
}