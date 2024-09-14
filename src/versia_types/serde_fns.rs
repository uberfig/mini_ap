use chrono::{DateTime, SecondsFormat};
// serde::ser::Error as that's the trait where the custom function comes from.
use serde::{ser::Error as SerError, de::Error as DeError, Deserialize, Deserializer, Serializer};


pub fn deserialize_time<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let input = String::deserialize(deserializer)?;
    let Ok(time) = DateTime::parse_from_rfc3339(&input) else {
        return Err(D::Error::custom("malformed created_at"));
    };
    Ok(time.timestamp_millis())
}

pub fn serialize_time<S>(x: &i64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let Some(time) = DateTime::from_timestamp_millis(*x) else {
        return Err(S::Error::custom("invalid timestamp"));
    };
    s.serialize_str(&time.to_rfc3339_opts(SecondsFormat::Secs, true))
}