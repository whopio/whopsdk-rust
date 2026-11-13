pub use crate::prelude::*;

/// Who the card application verifies, set only on the call that files it; sending it while the account's application is in progress or approved is a 400. `business` applies as the account with its approved KYB. `individual` applies as a person: assigned_user_id when given, otherwise the account owner, using the identity verified on the account. Omit it to apply as the business when its KYB is approved, and as an individual otherwise.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCardsRequestKind {
    Individual,
    Business,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCardsRequestKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Individual => serializer.serialize_str("individual"),
            Self::Business => serializer.serialize_str("business"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCardsRequestKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "individual" => Ok(Self::Individual),
            "business" => Ok(Self::Business),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCardsRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Individual => write!(f, "individual"),
            Self::Business => write!(f, "business"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
