pub use crate::prelude::*;

/// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Only changeable before the campaign is live on the ad network; switching to `ad_campaign` requires budget_amount in the same request, and switching to `ad_group` clears the campaign budget.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdCampaignsRequestBudgetOptimization {
    AdCampaign,
    AdGroup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdCampaignsRequestBudgetOptimization {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdCampaign => serializer.serialize_str("ad_campaign"),
            Self::AdGroup => serializer.serialize_str("ad_group"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdCampaignsRequestBudgetOptimization {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_campaign" => Ok(Self::AdCampaign),
            "ad_group" => Ok(Self::AdGroup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdCampaignsRequestBudgetOptimization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdCampaign => write!(f, "ad_campaign"),
            Self::AdGroup => write!(f, "ad_group"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
