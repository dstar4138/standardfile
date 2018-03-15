use chrono::{Utc,NaiveDateTime};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use uuid::Uuid;


/// Get the current UTC time in a consistent way.
pub fn current_time() -> NaiveDateTime {
    Utc::now().naive_utc()
}

/// Generate UUIDs consistent with default implementation.
pub fn new_uuid() -> String {
    // ruby-server uses SecureRandom.uuid
    Uuid::new_v4().to_string()
}

/// Simple digest for salt generation etc.
pub fn sha1_digest(msgs: Vec<&String>) -> String {
    let mut hasher = Sha1::new();
    for s in msgs.iter() {
        hasher.input_str(s.as_str());
    }
    hasher.result_str()
}
