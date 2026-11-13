pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSupportedMethodsRequestPartyType {
    FirstParty,
    ThirdParty,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSupportedMethodsRequestPartyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FirstParty => serializer.serialize_str("first_party"),
            Self::ThirdParty => serializer.serialize_str("third_party"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSupportedMethodsRequestPartyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first_party" => Ok(Self::FirstParty),
            "third_party" => Ok(Self::ThirdParty),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSupportedMethodsRequestPartyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstParty => write!(f, "first_party"),
            Self::ThirdParty => write!(f, "third_party"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
