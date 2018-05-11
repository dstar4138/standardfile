use actix_web::{
    HttpRequest, HttpResponse,
    FutureResponse, AsyncResponder,
    Json, State, Either, ResponseError,
};
use actix_web::middleware::identity::RequestIdentity;
use chrono::{Duration};
use futures::{Future};

use api::{
    errors::SFError,
    ServiceState, get_user_agent,
};
use util::{current_time};
use super::{
    SyncRequest, SyncResponse, IsDateTime,
    pagination::PaginationToken,
    minify_items, MinimalItem,
};
use db::GetAndUpdateItems;
use backend_core::models::Item;

static DEFAULT_LIMIT : i64 = 100_000;

use db::StandardFileResult;

pub fn sync(
    req: HttpRequest<ServiceState>,
    sync_req: Json<SyncRequest>,
    state: State<ServiceState>
) ->
             Either<FutureResponse<HttpResponse>, HttpResponse>
{
    let user_uuid = match req.identity() {
        None => return Either::B(SFError::InvalidCredentials.error_response()),
        Some(uuid) => uuid.to_string()
    };
    let user_agent = get_user_agent(&req);
    let items = sync_req.items.iter()
        .map(|&ref item: &MinimalItem| maximize_item(&user_uuid, &user_agent, item))
        .map(|item: Item| update_for_deleted(item))
        .collect::<Vec<Item>>();
    let sync_request = sync_req.into_inner();

    Either::A(
        state.db
        .send(build_query(&user_uuid, items.clone(), &sync_request))
        .from_err()
        .and_then(move |res: StandardFileResult<Option<Vec<Item>>>| match res {
            Err(_) => Err(SFError::InvalidCredentials.into()),
            Ok(optional_items) => {
                let retrieved_items = minify_items(optional_items);
                let cursor_token = retrieved_items.last().map(|last| {
                    let datetime = last.updated_at.to_datetime();
                    PaginationToken::from_datetime(datetime)
                });

                // add 1 second to avoid returning same object in subsequent sync, similar to ruby code.
                let last_updated = current_time() + Duration::seconds(1);
                let sync_token = PaginationToken::from_datetime(last_updated);

                let result = SyncResponse {
                    saved_items: sync_request.items.clone(),
                    unsaved: vec![],

                    retrieved_items,
                    cursor_token,
                    sync_token,
                };
                Ok(HttpResponse::Ok().json(result))
            }
        })
        .responder())
}

fn build_query(user_uuid: &String, items: Vec<Item>, request: &SyncRequest) -> GetAndUpdateItems {
    let (in_sync_token, in_cursor_token, limit) =
        (request.sync_token, request.cursor_token, request.limit.unwrap_or(DEFAULT_LIMIT));
    let (datetime, is_inclusive) = match (in_cursor_token, in_sync_token) {
        (Some(cursor_token), _) => {
            let datetime = cursor_token.to_datetime();
            debug!("Using cursor_token, {:?}", datetime);
            (Some(datetime), false)
        },
        (None, Some(sync_token)) => {
            let datetime = sync_token.to_datetime();
            debug!("Using sync_token, {}", datetime);
            (Some(datetime), true)
        },
        (None, None) => (None, false)
    };
    GetAndUpdateItems {
        items,
        user_uuid: user_uuid.to_string(),
        limit,
        datetime,
        is_inclusive
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