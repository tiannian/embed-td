//! An alternative timestamp serialization/deserialization mechanism for
//! RFC3339-compatible timestamps to that provided by the `tendermint-proto`
//! crate.

use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::utils::to_rfc3339_nanos;

/// Serialize from `Time` into `String`
pub fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = to_rfc3339_nanos(value);
    serializer.serialize_str(&s)
}

/// Deserialize `String` into `Time`
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let data = OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)?;

    Ok(data)
}
