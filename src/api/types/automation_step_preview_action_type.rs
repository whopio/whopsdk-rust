pub use crate::prelude::*;

/// The step's action type: `email` or `message`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationStepPreviewActionType {
    Email,
    Message,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationStepPreviewActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Message => serializer.serialize_str("message"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationStepPreviewActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "message" => Ok(Self::Message),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationStepPreviewActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Message => write!(f, "message"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
