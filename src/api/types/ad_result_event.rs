pub use crate::prelude::*;

/// The Whop pixel conversion event whose attributed count represents results — the optimization goal, or the highest-volume attributed event for campaigns that budget per ad group. Null when the goal isn't a Whop-attributed event.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdResultEvent {
    Purchase,
    Lead,
    Schedule,
    SubmitApplication,
    Contact,
    CompleteRegistration,
    ViewContent,
    AddToCart,
    Custom,
    MessagingConversation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdResultEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::Lead => serializer.serialize_str("lead"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::SubmitApplication => serializer.serialize_str("submit_application"),
            Self::Contact => serializer.serialize_str("contact"),
            Self::CompleteRegistration => serializer.serialize_str("complete_registration"),
            Self::ViewContent => serializer.serialize_str("view_content"),
            Self::AddToCart => serializer.serialize_str("add_to_cart"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::MessagingConversation => serializer.serialize_str("messaging_conversation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdResultEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "purchase" => Ok(Self::Purchase),
            "lead" => Ok(Self::Lead),
            "schedule" => Ok(Self::Schedule),
            "submit_application" => Ok(Self::SubmitApplication),
            "contact" => Ok(Self::Contact),
            "complete_registration" => Ok(Self::CompleteRegistration),
            "view_content" => Ok(Self::ViewContent),
            "add_to_cart" => Ok(Self::AddToCart),
            "custom" => Ok(Self::Custom),
            "messaging_conversation" => Ok(Self::MessagingConversation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdResultEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Purchase => write!(f, "purchase"),
            Self::Lead => write!(f, "lead"),
            Self::Schedule => write!(f, "schedule"),
            Self::SubmitApplication => write!(f, "submit_application"),
            Self::Contact => write!(f, "contact"),
            Self::CompleteRegistration => write!(f, "complete_registration"),
            Self::ViewContent => write!(f, "view_content"),
            Self::AddToCart => write!(f, "add_to_cart"),
            Self::Custom => write!(f, "custom"),
            Self::MessagingConversation => write!(f, "messaging_conversation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
