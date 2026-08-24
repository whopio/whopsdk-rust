pub use crate::prelude::*;

/// Who calculates and remits tax for the account: `whop` (Whop calculates and remits), `self` (Whop calculates; the account collects and remits), or `none` (neither; the account is responsible). `null` until the account enrolls in the Whop tax service.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountTaxRemittedBy {
    Whop,
    Self_,
    None,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountTaxRemittedBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Whop => serializer.serialize_str("whop"),
            Self::Self_ => serializer.serialize_str("self"),
            Self::None => serializer.serialize_str("none"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountTaxRemittedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "whop" => Ok(Self::Whop),
            "self" => Ok(Self::Self_),
            "none" => Ok(Self::None),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountTaxRemittedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Whop => write!(f, "whop"),
            Self::Self_ => write!(f, "self"),
            Self::None => write!(f, "none"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
