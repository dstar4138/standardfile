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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_time() {
        use std::{thread,time};
        let a = current_time();
        thread::sleep(time::Duration::from_millis(1000));
        let b = current_time();
        assert!(b.timestamp() > a.timestamp());
    }

    #[test]
    fn test_new_uuid() {
        use uuid::Uuid;
        let a = new_uuid();
        assert!(Uuid::parse_str(a.as_str()).is_ok());
        let b = new_uuid();
        assert!(Uuid::parse_str(b.as_str()).is_ok());
        assert_ne!(a,b);
    }

    #[test]
    fn test_sha1_digest() {
        let (hello,world) = ("hello".to_string(),"world".to_string());
        let input = vec![
            &hello, &world
        ];
        let expected_output = "6adfb183a4a2c94a2f92dab5ade762a47889a5a1".to_string();
        assert_eq!(expected_output, sha1_digest(input));
    }
}