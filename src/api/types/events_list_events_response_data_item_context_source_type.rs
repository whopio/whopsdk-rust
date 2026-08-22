pub use crate::prelude::*;

/// How this event counts as an acquisition touch, using the same rule attribution credits a conversion with. `ad_click` and `lead_form` resolved to a Whop ad; `external_ad_click` is a paid click on a campaign run outside Whop; `referrer` is organic. Null when the event is not a touch.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEventsResponseDataItemContextSourceType {
    AdClick,
    LeadForm,
    ExternalAdClick,
    Referrer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEventsResponseDataItemContextSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdClick => serializer.serialize_str("ad_click"),
            Self::LeadForm => serializer.serialize_str("lead_form"),
            Self::ExternalAdClick => serializer.serialize_str("external_ad_click"),
            Self::Referrer => serializer.serialize_str("referrer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEventsResponseDataItemContextSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_click" => Ok(Self::AdClick),
            "lead_form" => Ok(Self::LeadForm),
            "external_ad_click" => Ok(Self::ExternalAdClick),
            "referrer" => Ok(Self::Referrer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEventsResponseDataItemContextSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdClick => write!(f, "ad_click"),
            Self::LeadForm => write!(f, "lead_form"),
            Self::ExternalAdClick => write!(f, "external_ad_click"),
            Self::Referrer => write!(f, "referrer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
