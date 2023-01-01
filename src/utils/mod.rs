use serde::{Deserialize, Deserializer};

pub(crate) mod snowflake;
pub(crate) mod discord;
pub(crate) mod jwt;
pub(crate) mod cache_control;
pub(crate) mod raw_status;
pub(crate) mod cors;

// Any value that is present is considered Some value, including null.
pub(crate) fn deserialize_missing<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}
