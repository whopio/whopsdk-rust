pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListResolutionCenterCasesRequestOrder {
    CreatedAt,
    ResponseDueAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListResolutionCenterCasesRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::ResponseDueAt => serializer.serialize_str("response_due_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListResolutionCenterCasesRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "response_due_at" => Ok(Self::ResponseDueAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListResolutionCenterCasesRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::ResponseDueAt => write!(f, "response_due_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
