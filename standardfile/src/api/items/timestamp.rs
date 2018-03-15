use chrono::NaiveDateTime;
use super::IsDateTime;
use util::current_time;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ZuluTimestamp {
    datetime: NaiveDateTime,
}

impl IsDateTime for ZuluTimestamp {
    fn to_datetime(&self) -> NaiveDateTime {
        self.datetime
    }
    fn from_datetime(datetime: NaiveDateTime) -> ZuluTimestamp {
        ZuluTimestamp {
            datetime
        }
    }
}

impl Default for ZuluTimestamp {
    fn default() -> Self {
        ZuluTimestamp {
            datetime: current_time()
        }
    }
}

pub mod serde {
    use std::fmt;
    use super::ZuluTimestamp;
    use chrono::NaiveDateTime;
    use serde::ser::{Serialize, Serializer};
    use serde::de::{self,Visitor,Unexpected,Deserialize,Deserializer};

    static RFC3339_FORMAT  : &'static str = "%Y-%m-%dT%H:%M:%S%.fZ";

    impl Serialize for ZuluTimestamp {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where S: Serializer
        {
            let rfc3339 = format!("{}",self.datetime.format(RFC3339_FORMAT));
            serializer.serialize_str(&rfc3339)
        }
    }

    impl<'de> Deserialize<'de> for ZuluTimestamp {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where D: Deserializer<'de>
        {
            deserializer.deserialize_str(ZuluTimestampVisitor)
        }
    }

    struct ZuluTimestampVisitor;
    impl<'de> Visitor<'de> for ZuluTimestampVisitor {
        type Value = ZuluTimestamp;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an RFC3339 str timestamp")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: de::Error,
        {
            match NaiveDateTime::parse_from_str(v, RFC3339_FORMAT) {
                Ok(datetime) => Ok(ZuluTimestamp{datetime}),
                Err(_) => Err(E::invalid_type(Unexpected::Str(v), &"Invalid RFC3339 format"))
            }
        }
    }
}