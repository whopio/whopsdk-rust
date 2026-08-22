pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePeopleResponseSourcesItemType {
    AdClick,
    LeadForm,
    ExternalAdClick,
    Referrer,
    Utm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePeopleResponseSourcesItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdClick => serializer.serialize_str("ad_click"),
            Self::LeadForm => serializer.serialize_str("lead_form"),
            Self::ExternalAdClick => serializer.serialize_str("external_ad_click"),
            Self::Referrer => serializer.serialize_str("referrer"),
            Self::Utm => serializer.serialize_str("utm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePeopleResponseSourcesItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_click" => Ok(Self::AdClick),
            "lead_form" => Ok(Self::LeadForm),
            "external_ad_click" => Ok(Self::ExternalAdClick),
            "referrer" => Ok(Self::Referrer),
            "utm" => Ok(Self::Utm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePeopleResponseSourcesItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdClick => write!(f, "ad_click"),
            Self::LeadForm => write!(f, "lead_form"),
            Self::ExternalAdClick => write!(f, "external_ad_click"),
            Self::Referrer => write!(f, "referrer"),
            Self::Utm => write!(f, "utm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
