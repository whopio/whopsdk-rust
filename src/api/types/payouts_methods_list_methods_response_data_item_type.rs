pub use crate::prelude::*;

/// The kind of payout method. `external` sends funds out of Whop. `whop_card` moves funds onto the account's own Whop Card. A `whop_card` method takes standard-speed payouts only, and cannot be updated, reconnected, or deleted.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMethodsResponseDataItemType {
    External,
    WhopCard,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMethodsResponseDataItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::External => serializer.serialize_str("external"),
            Self::WhopCard => serializer.serialize_str("whop_card"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMethodsResponseDataItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "external" => Ok(Self::External),
            "whop_card" => Ok(Self::WhopCard),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMethodsResponseDataItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::External => write!(f, "external"),
            Self::WhopCard => write!(f, "whop_card"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
