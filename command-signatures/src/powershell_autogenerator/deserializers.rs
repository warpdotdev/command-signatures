use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Sometimes an empty string is placed in a field which is an object type. This will convert that
/// to a `None`.
pub(super) fn empty_string_is_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let value = Value::deserialize(deserializer)?;
    if value == Value::String("".to_string()) {
        return Ok(None);
    }
    T::deserialize(value)
        .map(|v| Some(v))
        .map_err(serde::de::Error::custom)
}

pub(super) fn literal_none_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| s.trim().to_lowercase() != "none"))
}

pub(super) fn string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(serde::de::Error::custom(format!("Unexpected value: {s}"))),
    }
}
