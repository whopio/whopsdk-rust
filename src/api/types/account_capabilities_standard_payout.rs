pub use crate::prelude::*;

/// Standard payouts to an external payout destination
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountCapabilitiesStandardPayout {
    Active,
    Inactive,
    Pending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountCapabilitiesStandardPayout {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Active => serializer.serialize_str("active"),
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountCapabilitiesStandardPayout {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "pending" => Ok(Self::Pending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountCapabilitiesStandardPayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Pending => write!(f, "pending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
