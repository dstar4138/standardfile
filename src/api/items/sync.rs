use iron::status;
use iron::prelude::*;
use chrono::{Duration};
use serde_json;
use serde_json::value::Value;

use super::timestamp::ZuluTimestamp;
use super::pagination::{PaginationToken};
use db::{get_connection,DbConnection,StandardFileStorage};
use models::Item;

static DEFAULT_LIMIT : i64 = 100_000;
static FULL_FAILURE_RESPONSE : &'static str = "{}";

use api::{
//    encode_error_msg,
    load_json_req_body,
    get_current_user_uuid,
    get_user_agent,
    ResultWithErrorResponse
};

use super::{
    SyncResponse,
    MinimalItem,
    IsDateTime
};

use util::current_time;

pub fn sync(req: &mut Request) -> IronResult<Response> {
    let conn = get_connection().expect("Unable to get db connection.");
    let response = match do_sync(req, &conn) {
        Err(error_msg) => error_msg,
        Ok(response) => response
    };
    info!("sync response: {:?}", response);
    Ok(Response::with(response))
}
fn do_sync(req: &mut Request, conn: &DbConnection) -> ResultWithErrorResponse<(status::Status, String)> {
    let user_uuid   = get_current_user_uuid(req, conn)?;
    let user_agent  = get_user_agent(req)?;
    let items       = get_sync_items(req);
    let sync_params = get_sync_params(req);
    info!("User attempting sync, {}, via user agent, '{}'.", user_uuid, user_agent);

    let (retrieved_items, cursor_token) = do_sync_get(&user_uuid, sync_params, conn);
    let (saved_items, unsaved)          = do_sync_save(&user_uuid, items, &user_agent, conn);

    // add 1 microsecond to avoid returning same object in subsequent sync, same as ruby code
    let last_updated = current_time() + Duration::microseconds(1);
    let sync_token = PaginationToken::from_datetime(last_updated);

    Ok((status::Ok, serde_json::to_string(&SyncResponse {
        retrieved_items,
        saved_items,
        unsaved,
        sync_token,
        cursor_token
    }).unwrap_or(FULL_FAILURE_RESPONSE.to_string())))
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
fn get_sync_params(req: &mut Request) -> (Option<PaginationToken>,Option<PaginationToken>,i64) {
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
fn unwrap_decode(val: Option<&Value>) -> Option<PaginationToken> {
    match val {
        None => None,
        Some(v) => serde_json::from_value(v.to_owned()).unwrap_or(None)
    }
}

fn do_sync_get(user_uuid:&String, sync_params: (Option<PaginationToken>,Option<PaginationToken>,i64), conn: &DbConnection) -> (Vec<MinimalItem>,Option<PaginationToken>) {
    let (in_sync_token, in_cursor_token, limit) = sync_params;
    let optional_items = match (in_cursor_token, in_sync_token) {
        (None,Some(sync_token)) => {
            let datetime = sync_token.to_datetime();
            info!("Using sync_token, {}",datetime);
            conn.get_items_older_or_equal_to(&datetime, user_uuid, limit)
        },
        (Some(cursor_token), _) => {
            let datetime = cursor_token.to_datetime();
            info!("Using cursor_token, {}", datetime);
            conn.get_items_older_than(&datetime, user_uuid, limit)
        },
        (None, None) => conn.get_items(user_uuid, limit)
    };
    let items = minify_items(optional_items);
    let cursor_token = match items.last() {
        None => None,
        Some(last) => {
            let datetime = last.updated_at.to_datetime();
            Some(PaginationToken::from_datetime(datetime))
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
        created_at: ZuluTimestamp::from_datetime(item.created_at.clone()),
        updated_at: ZuluTimestamp::from_datetime(item.updated_at.clone()),
    }
}
fn maximize_item(user_uuid: &String, last_user_agent: &String, item: &MinimalItem) -> Item {
    Item { //TODO: this looks dumb... is this seriously what i have to do?
        uuid: item.uuid.clone(),
        content: item.content.clone().into_bytes(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash: item.auth_hash.clone().unwrap_or("".to_string()),
        user_uuid: user_uuid.to_owned(),
        deleted: item.deleted.clone(),
        created_at: item.created_at.clone().to_datetime(),
        updated_at: item.updated_at.clone().to_datetime(),
        last_user_agent: Some(last_user_agent.to_owned()),
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
fn do_sync_save(user_uuid:&String, items: Vec<MinimalItem>, user_agent: &String, conn: &DbConnection) -> (Vec<MinimalItem>, Vec<MinimalItem>) {
    let (saved_items, unsaved_items) = items
        .iter()
        .map(|&ref item: &MinimalItem| maximize_item(user_uuid, user_agent,item))
        .map(|item: Item| update_for_deleted(item))
        .map(|item: Item| conn.add_or_update_item(item))
        .partition(|db_result: &Result<Item,Item>| db_result.is_ok());
    (unwrap(saved_items), unwrap(unsaved_items))
}