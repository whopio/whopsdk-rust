pub use crate::prelude::*;

/// The goal the campaign optimizes toward.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAdCampaignsRequestObjective {
    Awareness,
    Traffic,
    Engagement,
    Leads,
    Sales,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAdCampaignsRequestObjective {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Awareness => serializer.serialize_str("awareness"),
            Self::Traffic => serializer.serialize_str("traffic"),
            Self::Engagement => serializer.serialize_str("engagement"),
            Self::Leads => serializer.serialize_str("leads"),
            Self::Sales => serializer.serialize_str("sales"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAdCampaignsRequestObjective {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "awareness" => Ok(Self::Awareness),
            "traffic" => Ok(Self::Traffic),
            "engagement" => Ok(Self::Engagement),
            "leads" => Ok(Self::Leads),
            "sales" => Ok(Self::Sales),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAdCampaignsRequestObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Awareness => write!(f, "awareness"),
            Self::Traffic => write!(f, "traffic"),
            Self::Engagement => write!(f, "engagement"),
            Self::Leads => write!(f, "leads"),
            Self::Sales => write!(f, "sales"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
