pub use crate::prelude::*;

/// Interaction that qualifies a person for this rule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceEngagementFacebookPageRuleEvent {
    Engaged,
    Visited,
    Liked,
    Messaged,
    CtaClicked,
    Saved,
    PostInteraction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceEngagementFacebookPageRuleEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Engaged => serializer.serialize_str("engaged"),
            Self::Visited => serializer.serialize_str("visited"),
            Self::Liked => serializer.serialize_str("liked"),
            Self::Messaged => serializer.serialize_str("messaged"),
            Self::CtaClicked => serializer.serialize_str("cta_clicked"),
            Self::Saved => serializer.serialize_str("saved"),
            Self::PostInteraction => serializer.serialize_str("post_interaction"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceEngagementFacebookPageRuleEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "engaged" => Ok(Self::Engaged),
            "visited" => Ok(Self::Visited),
            "liked" => Ok(Self::Liked),
            "messaged" => Ok(Self::Messaged),
            "cta_clicked" => Ok(Self::CtaClicked),
            "saved" => Ok(Self::Saved),
            "post_interaction" => Ok(Self::PostInteraction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceEngagementFacebookPageRuleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engaged => write!(f, "engaged"),
            Self::Visited => write!(f, "visited"),
            Self::Liked => write!(f, "liked"),
            Self::Messaged => write!(f, "messaged"),
            Self::CtaClicked => write!(f, "cta_clicked"),
            Self::Saved => write!(f, "saved"),
            Self::PostInteraction => write!(f, "post_interaction"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
