pub use crate::prelude::*;

/// The role of an affiliate override (standard or rev_share)
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliateOverrideRoles {
    Standard,
    RevShare,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffiliateOverrideRoles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::RevShare => serializer.serialize_str("rev_share"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffiliateOverrideRoles {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "rev_share" => Ok(Self::RevShare),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffiliateOverrideRoles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::RevShare => write!(f, "rev_share"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
