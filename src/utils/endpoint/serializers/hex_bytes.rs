//! Hash serialization with validation

use serde::{Deserialize, Deserializer, Serializer};

/// Deserialize hexstring into Hash
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let hexstring: String = Option::<String>::deserialize(deserializer)?.unwrap_or_default();

    let data = hex::decode(hexstring).map_err(serde::de::Error::custom)?;

    Ok(data)
}

/// Serialize from Hash into hexstring
pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_bytes = hex::encode_upper(value);
    serializer.serialize_str(&hex_bytes)
}
