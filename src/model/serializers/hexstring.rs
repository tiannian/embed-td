use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
    hex::decode(&string)
        .or_else(|_| hex::decode(&string))
        .map_err(serde::de::Error::custom)
}

/// Serialize from T into hexstring
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<[u8]>,
{
    let hex_bytes = hex::encode_upper(value.as_ref());
    serializer.serialize_str(&hex_bytes)
}
