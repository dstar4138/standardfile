use chrono::{Duration};
use actix_web::{HttpRequest, HttpMessage, Error, StatusCode, AsyncResponder};
use futures::{Future,IntoFuture};

use super::pagination::{PaginationToken};
use db::{get_connection,StandardFileStorage};
use backend_core::models::Item;

static DEFAULT_LIMIT : i64 = 100_000;

use api::{
    INVALID_CREDENTIALS,
    ResultObj, FutureResultObj, ErrorCode,
    return_ok, return_err,
    get_current_user_uuid,
    get_user_agent,
};

use super::{
    SyncRequest, SyncResponse,
    MinimalItem,
    IsDateTime,
};

use util::current_time;

pub fn sync(req: HttpRequest) -> FutureResultObj<SyncResponse> {
    let conn = get_connection().expect("Unable to get db connection");
    let user_uuid  =  match get_current_user_uuid(&req, &conn) {
        Err(err) => {
            info!("Invalid sync request due to unknown user");
            return Box::new(Ok(err).into_future())
        },
        Ok(id)   => id
    };
    let user_agent = get_user_agent(&req);
    req.json()
        .from_err()
        .then(|res : Result<SyncRequest, Error>| match res {
            Err(e) => {
                error!("Error: {}", e);
                Ok(return_err(ErrorCode(StatusCode::UNAUTHORIZED, INVALID_CREDENTIALS)))
            },
            Ok(request) => {
                let conn = get_connection().expect("Unable to get db connection");
                info!("[UserUuid: {:?}].", user_uuid);
                Ok(do_sync(request, user_uuid, user_agent, &conn))
            },
        }).responder()
}

fn do_sync(req: SyncRequest, user_uuid: String, user_agent: String, conn: &Box<StandardFileStorage>) -> ResultObj<SyncResponse> {
    let (retrieved_items, cursor_token) = do_sync_get(&user_uuid, &req, conn);
    let (saved_items, unsaved)          = do_sync_save(&user_uuid, &req.items, &user_agent, conn);

    // add 1 microsecond to avoid returning same object in subsequent sync, same as ruby code
    let last_updated = current_time() + Duration::microseconds(1);
    let sync_token = PaginationToken::from_datetime(last_updated);

    return_ok(SyncResponse {
        retrieved_items,
        saved_items,
        unsaved,
        sync_token,
        cursor_token
    })
}

fn do_sync_get(user_uuid:&String, request: &SyncRequest, conn: &Box<StandardFileStorage>) -> (Vec<MinimalItem>,Option<PaginationToken>) {
    let (in_sync_token, in_cursor_token, limit) =
            (request.sync_token, request.cursor_token, request.limit.unwrap_or(DEFAULT_LIMIT));
    let optional_items = match (in_cursor_token, in_sync_token) {
        (None,Some(sync_token)) => {
            let datetime = sync_token.to_datetime();
            debug!("Using sync_token, {}",datetime);
            conn.get_items_older_or_equal_to(&datetime, user_uuid, limit)
        },
        (Some(cursor_token), _) => {
            let datetime = cursor_token.to_datetime();
            debug!("Using cursor_token, {}", datetime);
            conn.get_items_older_than(&datetime, user_uuid, limit)
        },
        (None, None) => conn.get_items(user_uuid, limit)
    };
    let items = minify_items(optional_items);
    let cursor_token = items.last().map(|last| {
            let datetime = last.updated_at.to_datetime();
            PaginationToken::from_datetime(datetime)
        }
    );
    (items,cursor_token)
}
fn minify_items(optional_items: Option<Vec<Item>>) -> Vec<MinimalItem> {
    match optional_items {
        None => vec![],
        Some(items) =>
            items.iter().map(|&ref item: &Item| MinimalItem::from(item)).collect()
    }
}

fn maximize_item(user_uuid: &String, last_user_agent: &String, item: &MinimalItem) -> Item {
    Item { //TODO: this looks dumb... is this seriously what i have to do?
        uuid:         item.uuid.clone(),
        content:      item.content.clone().into_bytes(),
        content_type: item.content_type.clone(),
        enc_item_key: item.enc_item_key.clone(),
        auth_hash:    item.auth_hash.clone().unwrap_or("".to_string()),
        user_uuid:    user_uuid.to_owned(),
        deleted:      item.deleted.clone(),
        created_at:   item.created_at.clone().to_datetime(),
        updated_at:   item.updated_at.clone().to_datetime(),
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
        .map(|&ref item: &Item | MinimalItem::from(item))
        .collect()
}
fn do_sync_save(user_uuid:&String, items: &Vec<MinimalItem>, user_agent: &String, conn: &Box<StandardFileStorage>) -> (Vec<MinimalItem>, Vec<MinimalItem>) {
    let (saved_items, unsaved_items) = items
        .iter()
        .map(|&ref item: &MinimalItem| maximize_item(user_uuid, user_agent,item))
        .map(|item: Item| update_for_deleted(item))
        .map(|item: Item| conn.add_or_update_item(item))
        .partition(|db_result: &Result<Item,Item>| db_result.is_ok());
    (unwrap(saved_items), unwrap(unsaved_items))
}