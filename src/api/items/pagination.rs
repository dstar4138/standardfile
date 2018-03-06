use chrono::{NaiveDateTime};
use super::IsDateTime;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct PaginationToken {
    version: u32,
    datetime: NaiveDateTime,
}

impl IsDateTime for PaginationToken {
    fn to_datetime(&self) -> NaiveDateTime {
        self.datetime
    }
    fn from_datetime(datetime: NaiveDateTime) -> PaginationToken {
        PaginationToken {
            version: 2,
            datetime
        }
    }
}

pub mod serde {
    use super::PaginationToken;
    use base64;
    use chrono::{NaiveDateTime};
    use std::num::ParseIntError;
    use std::fmt;
    use serde::ser::{Serialize, Serializer};
    use serde::de::{self,Visitor,Unexpected,Deserialize,Deserializer};

    static TOKEN_FORMAT_V1: &'static str = "%s";
    static TOKEN_FORMAT_V2: &'static str = "%s%.9f";

    impl Serialize for PaginationToken {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            let timestamp = match self.version {
                1 => self.datetime.format(TOKEN_FORMAT_V1),
                _ => self.datetime.format(TOKEN_FORMAT_V2)
            };
            let encoded:String = base64::encode(&format!("{}:{}", self.version, timestamp));
            serializer.serialize_str(encoded.as_str())
        }
    }

    impl<'de> Deserialize<'de> for PaginationToken {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
            D: Deserializer<'de> {
            deserializer.deserialize_str(PaginationTokenVisitor)
        }
    }

    struct PaginationTokenVisitor;
    impl<'de> Visitor<'de> for PaginationTokenVisitor {
        type Value = PaginationToken;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a base64 encoded str containing a versioned timestamp")
        }

        fn visit_str<E>(self, token: &str) -> Result<Self::Value, E>
            where E: de::Error,
        {
            let bytes = base64::decode(token).unwrap();
            let tokens = String::from_utf8(bytes).unwrap();
            let vec: Vec<&str> = tokens.split(":").collect();
            if vec.len() != 2 {
                return Err(E::invalid_type(Unexpected::Str(token), &"a bad format token"));
            }

            let version = vec.get(0).unwrap().parse::<u32>().unwrap();
            let timestamp = vec.get(1).unwrap();
            info!("Attempting to decode timestamp in token: v={}, t={}", version, timestamp);
            match version {
                1 => match NaiveDateTime::parse_from_str(timestamp, TOKEN_FORMAT_V1) {
                    Err(e) => {
                        error!("failure to parse v1 token: {}", e);
                        Err(E::invalid_type(Unexpected::Str(token), &"poor format for version 1"))
                    },
                    Ok(datetime) => Ok(PaginationToken { version, datetime })
                }
                _ => match parse_v2(timestamp) {
                    Err(_) => {
                        error!("failure to parse v2 token: {}", "parseIntError");
                        Err(E::invalid_type(Unexpected::Str(token), &"poor format for version 2"))
                    },
                    Ok(datetime) => Ok(PaginationToken { version, datetime })
                }
            }
        }
    }

    fn parse_v2(timestamp: &str) -> Result<NaiveDateTime, ParseIntError> {
        let stamp: &String = &timestamp.to_owned();
        let tokens: Vec<&str> = stamp.split(".").collect();
        let secs = tokens.get(0).unwrap().parse::<i64>()?;
        let nsecs = pad_zeros(tokens.get(1).unwrap(), 9).parse::<u32>()?;
        Ok(NaiveDateTime::from_timestamp(secs, nsecs))
    }

    fn pad_zeros(s: &str, size: i64) -> String {
        let mut new_string = String::new();
        new_string.push_str(s);
        let pad_len : i64 = size - (s.len() as i64);
        if pad_len < 0 {
            new_string.truncate(pad_len as usize);
        } else {
            for _ in 0..pad_len { new_string.push('0'); }
        }
        new_string
    }
}


