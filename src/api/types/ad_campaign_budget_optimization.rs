pub use crate::prelude::*;

/// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignBudgetOptimization {
    AdCampaign,
    AdGroup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdCampaignBudgetOptimization {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdCampaign => serializer.serialize_str("ad_campaign"),
            Self::AdGroup => serializer.serialize_str("ad_group"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdCampaignBudgetOptimization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_campaign" => Ok(Self::AdCampaign),
            "ad_group" => Ok(Self::AdGroup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdCampaignBudgetOptimization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdCampaign => write!(f, "ad_campaign"),
            Self::AdGroup => write!(f, "ad_group"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
