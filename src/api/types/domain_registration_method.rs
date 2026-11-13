pub use crate::prelude::*;

/// How the domain came to Whop: `registration` when it was bought through Whop, `transfer` when it was moved from another registrar.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainRegistrationMethod {
    Registration,
    Transfer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainRegistrationMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Registration => serializer.serialize_str("registration"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainRegistrationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "registration" => Ok(Self::Registration),
            "transfer" => Ok(Self::Transfer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainRegistrationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Registration => write!(f, "registration"),
            Self::Transfer => write!(f, "transfer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
