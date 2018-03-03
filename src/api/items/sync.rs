use base64;
use iron::status;
use iron::prelude::*;
use chrono::{NaiveDateTime,Duration};
use serde_json;
use serde_json::value::Value;

use std::num::ParseIntError;

use db;
use diesel::prelude::SqliteConnection;
use models::Item;

static TOKEN_FORMAT_V1 : &'static str = "%s";
static TOKEN_FORMAT_V2 : &'static str = "%s%.9f";
static DEFAULT_LIMIT : i64 = 100_000;

use api::{
//    encode_error_msg,
    load_json_req_body,
    get_current_user_uuid,
    get_user_agent
};

use super::{
    SyncError,
    SyncErrorKind,
    SyncResult,
    SyncResponse,
    MinimalItem,
    naivedatetime_to_rfc3339_string,
    rfc3339_string_to_naivedatetime
};

use util::current_time;

pub fn sync(req: &mut Request) -> IronResult<Response> {
    let user_uuid = match get_current_user_uuid(req) {
        Ok(val) => {
            info!("SYNC(User={:?})",val);
            val
        },
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
            info!("SYNC BODY: {:?}",hashmap);
            match hashmap.get("items") {
                None => vec![],
                Some(in_items) => {
                    info!("SYNC ITEMS: {:?}", in_items);
                    let values:Vec<Value> = in_items.as_array().unwrap().to_vec();
                    values.iter().map(
                        |val: &Value| {
                            match serde_json::from_value(val.to_owned()) {
                                Err(e) => {
                                    error!("ERR ITEM: {:?}",val);
                                    panic!(e)
                                },
                                Ok(item) => item
                            }
                        }
                    ).collect()
                }
            }
        }
    }
}
/// in_sync_token, in_cursor_token, in_limit
fn get_sync_params(req: &mut Request) -> (Option<String>,Option<String>,i64) {
    match load_json_req_body(req) {
        Err(_) => (None,None,DEFAULT_LIMIT),
        Ok(ref hashmap) => {
            let in_sync_token = unwrap_decode(hashmap.get("sync_token"));
            let in_cursor_token = unwrap_decode(hashmap.get("cursor_token"));
            match hashmap.get("limit") {
                None => (in_sync_token, in_cursor_token, DEFAULT_LIMIT),
                Some(v) => {
                    let limit = v.as_i64().unwrap_or(DEFAULT_LIMIT);
                    info!("SYNC(sync_token='{:?}',cursor_token='{:?}',limit={})",in_sync_token,in_cursor_token,limit);
                    (in_sync_token, in_cursor_token, limit)
                }
           }
        }
    }
}
fn unwrap_decode(val: Option<&Value>) -> Option<String> {
    match val {
        None => None,
        Some(v) => serde_json::from_value(v.to_owned()).unwrap_or(None)
    }
}
fn generate_sync_token(last_update: &NaiveDateTime) -> String {
    let version : u32 = 2;
    let timestamp = last_update.format(TOKEN_FORMAT_V2);
    base64::encode(&format!("{}:{}",version,timestamp))
}
fn get_last_update_from_sync_token(sync_token:String) -> SyncResult<NaiveDateTime> {
    let bytes = base64::decode(&sync_token).unwrap();
    let token = String::from_utf8(bytes).unwrap();
    let vec : Vec<&str> = token.split(":").collect();
    if vec.len() != 2 {
        return Err(SyncError(SyncErrorKind::InvalidToken));
    }

    let version = vec.get(0).unwrap().parse::<u32>().unwrap();
    let timestamp = vec.get(1).unwrap();
    info!("Attempting to decode timestamp in token: v={}, t={}", version, timestamp);
    match version {
        1 => match NaiveDateTime::parse_from_str(timestamp, TOKEN_FORMAT_V1) {
            Err(e) => {
                error!("failure to parse v1 token: {}",e);
                Err(SyncError(SyncErrorKind::ParseErrorToken))
            },
            Ok(datetime) => Ok(datetime)
        }
        _ => match parse_v2(timestamp) {
            Err(_) => {
                error!("failure to parse v2 token: {}","parseIntError");
                Err(SyncError(SyncErrorKind::ParseErrorToken))
            },
            Ok(datetime) => Ok(datetime)
        }
    }
}
fn parse_v2(timestamp: &str) -> Result<NaiveDateTime,ParseIntError> {
    let stamp : &String = &timestamp.to_owned();
    let tokens: Vec<&str>  = stamp.split(".").collect();
    let secs = tokens.get(0).unwrap().parse::<i64>()?;
    let nsecs = tokens.get(1).unwrap().parse::<u32>()?;
    Ok(NaiveDateTime::from_timestamp(secs,nsecs))
}

fn do_sync_get(user_uuid:&String, sync_params: (Option<String>,Option<String>,i64), conn: &SqliteConnection) -> (Vec<MinimalItem>,Option<String>) {
    let (in_sync_token, in_cursor_token, limit) = sync_params;
    let optional_items = match (in_cursor_token, in_sync_token) {
        (None,Some(sync_token)) =>
            match get_last_update_from_sync_token(sync_token) {
                Ok(datetime) =>{
                    info!("Using sync_token, {}",datetime);
                    db::get_items_older_or_equal_to(conn, &datetime, user_uuid, limit)
                },
                Err(e) => {
                    info!("tried to use sync_token, {}",e);
                    db::get_items(conn, user_uuid, limit)
                }
            },
        (Some(cursor_token), _) =>
            match get_last_update_from_sync_token(cursor_token) {
                Ok(datetime) => {
                    info!("Using cursor_token, {}", datetime);
                    db::get_items_older_than(conn, &datetime, user_uuid, limit)
                },
                Err(e) => {
                    info!("tried to use cursor_token, {}", e);
                    db::get_items(conn, user_uuid, limit)
                }
            },
        (None, None) => db::get_items(conn, user_uuid, limit)
    };
    let items = minify_items(optional_items);
    let cursor_token = match items.last() {
        None => None,
        Some(last) => {
            let datetime = rfc3339_string_to_naivedatetime(last.updated_at.clone()).unwrap_or(current_time());
            Some(generate_sync_token(&datetime))
        }
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
        content: String::from_utf8(item.content.clone()).unwrap(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash: Some(item.auth_hash.clone()),
        deleted: item.deleted.clone(),
        created_at: naivedatetime_to_rfc3339_string(item.created_at.clone()),
        updated_at: naivedatetime_to_rfc3339_string(item.updated_at.clone()),
    }
}
fn maximize_item(user_uuid: &String, last_user_agent: &Option<String>, item: &MinimalItem) -> Item {
    Item { //TODO: this looks dumb... is this seriously what i have to do?
        uuid: item.uuid.clone(),
        content: item.content.clone().into_bytes(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash: item.auth_hash.clone().unwrap_or("".to_string()),
        user_uuid: user_uuid.to_owned(),
        deleted: item.deleted.clone(),
        created_at: rfc3339_string_to_naivedatetime(item.created_at.clone()).unwrap_or(current_time()),
        updated_at: rfc3339_string_to_naivedatetime(item.updated_at.clone()).unwrap_or(current_time()),
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