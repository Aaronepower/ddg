use std::fmt;

use serde::{de, Deserializer};

/// An icon associated with a URL. Icon's can be bigger than the dimensions
/// specified in the struct.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Icon {
    /// The **suggested** height of the icon.
    #[serde(rename="Height", deserialize_with="deserialize_u64_or_empty_string")]
    pub height: u64,
    /// The url to the icon.
    #[serde(rename="URL")]
    pub url: String,
    /// The **suggested** width of the icon.
    #[serde(rename="Width", deserialize_with="deserialize_u64_or_empty_string")]
    pub width: u64,
}

// from: http://stackoverflow.com/questions/37870428/serde-handle-value-being-two-different-types
struct DeserializeU64OrEmptyStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeU64OrEmptyStringVisitor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an unsigned integer or an empty string.")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: de::Error {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error
    {
        if v.is_empty() {
            Ok(0)
        } else {
            Err(E::invalid_value(de::Unexpected::Str(v), &self))
        }
    }
}

fn deserialize_u64_or_empty_string<'de, D>(deserializer: D)
    -> Result<u64, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_any(DeserializeU64OrEmptyStringVisitor)
}
