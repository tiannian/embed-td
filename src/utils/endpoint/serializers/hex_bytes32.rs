//! Hash serialization with validation

use serde::{Deserialize, Deserializer, Serializer};

/// Deserialize hexstring into Hash
pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    let hexstring: String = Option::<String>::deserialize(deserializer)?.unwrap_or_default();

    let mut bytes32 = [0u8; 32];

    let data = hex::decode(hexstring).map_err(serde::de::Error::custom)?;

    bytes32.clone_from_slice(&data);

    Ok(bytes32)
}

/// Serialize from Hash into hexstring
pub fn serialize<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_bytes = hex::encode_upper(value);
    serializer.serialize_str(&hex_bytes)
}
