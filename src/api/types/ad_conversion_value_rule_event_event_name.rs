pub use crate::prelude::*;

/// Event to adjust. Purchase includes Whop purchases and external purchase events.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdConversionValueRuleEventEventName {
    Purchase,
    Subscribe,
    StartTrial,
    Lead,
    CompleteRegistration,
    SubmitApplication,
    Schedule,
    Contact,
    ViewContent,
    AddToCart,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdConversionValueRuleEventEventName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::Subscribe => serializer.serialize_str("subscribe"),
            Self::StartTrial => serializer.serialize_str("start_trial"),
            Self::Lead => serializer.serialize_str("lead"),
            Self::CompleteRegistration => serializer.serialize_str("complete_registration"),
            Self::SubmitApplication => serializer.serialize_str("submit_application"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::Contact => serializer.serialize_str("contact"),
            Self::ViewContent => serializer.serialize_str("view_content"),
            Self::AddToCart => serializer.serialize_str("add_to_cart"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdConversionValueRuleEventEventName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "purchase" => Ok(Self::Purchase),
            "subscribe" => Ok(Self::Subscribe),
            "start_trial" => Ok(Self::StartTrial),
            "lead" => Ok(Self::Lead),
            "complete_registration" => Ok(Self::CompleteRegistration),
            "submit_application" => Ok(Self::SubmitApplication),
            "schedule" => Ok(Self::Schedule),
            "contact" => Ok(Self::Contact),
            "view_content" => Ok(Self::ViewContent),
            "add_to_cart" => Ok(Self::AddToCart),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdConversionValueRuleEventEventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Purchase => write!(f, "purchase"),
            Self::Subscribe => write!(f, "subscribe"),
            Self::StartTrial => write!(f, "start_trial"),
            Self::Lead => write!(f, "lead"),
            Self::CompleteRegistration => write!(f, "complete_registration"),
            Self::SubmitApplication => write!(f, "submit_application"),
            Self::Schedule => write!(f, "schedule"),
            Self::Contact => write!(f, "contact"),
            Self::ViewContent => write!(f, "view_content"),
            Self::AddToCart => write!(f, "add_to_cart"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
