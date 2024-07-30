use itertools::Itertools as _;
use serde::{de::Error as _, Deserialize, Deserializer};
use serde_json::Value;

use crate::powershell_autogenerator::ParameterPosition;

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
    let s = Option::<String>::deserialize(deserializer)?;
    Ok(s.filter(|s| s.trim().to_lowercase() != "none"))
}

pub(super) fn literal_none_is_empty<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.trim().to_lowercase() == "none" {
        return Ok(vec![]);
    }
    Ok(s.split(", ").map(ToOwned::to_owned).collect_vec())
}

pub(super) fn string_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(D::Error::custom(format!("Unexpected value: {s}"))),
    }
}

impl<'de> Deserialize<'de> for ParameterPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.to_lowercase() == "named" {
            return Ok(Self::Named);
        }

        match s.parse::<usize>() {
            Ok(i) => Ok(Self::Index(i)),
            Err(_) => Err(D::Error::custom(format!(
                "Invalid value {s:?}. Expected 'named' or an integer."
            ))),
        }
    }
}
