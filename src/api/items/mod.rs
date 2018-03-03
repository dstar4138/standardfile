mod sync;
pub use self::sync::sync;

use std::fmt;
use std::error::Error;
use chrono::{NaiveDateTime,ParseError};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct SyncError(SyncErrorKind);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum SyncErrorKind {
    /// The Sync or Cursor token was invalid, either missing the version or timestamp.
    InvalidToken,

    /// Failure to parse token, may not meet expectations
    ParseErrorToken
}

impl Error for SyncError {
    fn description(&self) -> &str {
        match self.0 {
            SyncErrorKind::InvalidToken => "Sync or cursor token was invalid.",
            SyncErrorKind::ParseErrorToken => "Unable to parse timestamp for sync or cursor token.",
        }
    }
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

type SyncResult<T> = Result<T, SyncError>;

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    retrieved_items: Vec<MinimalItem>,
    saved_items: Vec<MinimalItem>,
    unsaved: Vec<MinimalItem>,
    sync_token: String,
    cursor_token: Option<String>,
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
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

static RFC3339_FORMAT  : &'static str = "%Y-%m-%dT%H:%M:%S%.fZ";

//TODO: find a way to make these baked in.
fn naivedatetime_to_rfc3339_string(datetime: NaiveDateTime) -> String {
    format!("{}",datetime.format(RFC3339_FORMAT)).to_string()
}
fn rfc3339_string_to_naivedatetime(string_date_time: String) -> Result<NaiveDateTime,ParseError> {
    NaiveDateTime::parse_from_str(string_date_time.as_str(), RFC3339_FORMAT)
}