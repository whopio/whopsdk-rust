pub use crate::prelude::*;

/// What the step does: `wait` holds the member for `config.delay_seconds`, `email` emails them, and `message` sends them a message from a team member. A draft step can switch between `email` and `message`, which clears `config.sender_id`. A `wait` can't become a sending step, or the other way round.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationStepActionType {
    Wait,
    Email,
    Message,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationStepActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Wait => serializer.serialize_str("wait"),
            Self::Email => serializer.serialize_str("email"),
            Self::Message => serializer.serialize_str("message"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationStepActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wait" => Ok(Self::Wait),
            "email" => Ok(Self::Email),
            "message" => Ok(Self::Message),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationStepActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wait => write!(f, "wait"),
            Self::Email => write!(f, "email"),
            Self::Message => write!(f, "message"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
