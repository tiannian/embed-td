use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// Deserialize string into T
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: core::str::FromStr,
    <T as core::str::FromStr>::Err: core::fmt::Display,
{
    String::deserialize(deserializer)?
        .parse::<T>()
        .map_err(|e| D::Error::custom(format!("{}", e)))
}

/// Serialize from T into string
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: core::fmt::Display,
{
    format!("{}", value).serialize(serializer)
}
