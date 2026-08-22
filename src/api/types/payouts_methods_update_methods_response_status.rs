pub use crate::prelude::*;

/// `created` — saved, no payout has completed through it yet. `active` — a payout through it completed. `broken` — a payout through it failed with an error attributable to the method, and it no longer accepts payouts; a later successful payout returns it to `active`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateMethodsResponseStatus {
    Created,
    Active,
    Broken,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateMethodsResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Created => serializer.serialize_str("created"),
            Self::Active => serializer.serialize_str("active"),
            Self::Broken => serializer.serialize_str("broken"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateMethodsResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created" => Ok(Self::Created),
            "active" => Ok(Self::Active),
            "broken" => Ok(Self::Broken),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateMethodsResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Active => write!(f, "active"),
            Self::Broken => write!(f, "broken"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
