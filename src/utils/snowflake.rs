use core::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

pub type Snowflake = i64;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct ApiSnowflake(pub Snowflake);

impl Deref for ApiSnowflake {
    type Target = Snowflake;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for ApiSnowflake {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized {
        data.hash(state);
    }
}

impl PartialEq<Self> for ApiSnowflake {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for ApiSnowflake {}

impl Serialize for ApiSnowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ApiSnowflakeVisitor;

impl<'de> Visitor<'de> for ApiSnowflakeVisitor {
    type Value = ApiSnowflake;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a snowflake as a string")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
    {
        let i: i64 = value.parse().map_err(|_| de::Error::invalid_type(de::Unexpected::Str(value), &self))?;
        Ok(ApiSnowflake(i))
    }
}

impl<'de> Deserialize<'de> for ApiSnowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        deserializer.deserialize_identifier(ApiSnowflakeVisitor)
    }
}
