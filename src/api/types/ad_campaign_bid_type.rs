pub use crate::prelude::*;

/// How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, and `maximum_target` never bids above a cap.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignBidType {
    MinimumCost,
    AverageTarget,
    MaximumTarget,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdCampaignBidType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MinimumCost => serializer.serialize_str("minimum_cost"),
            Self::AverageTarget => serializer.serialize_str("average_target"),
            Self::MaximumTarget => serializer.serialize_str("maximum_target"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdCampaignBidType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "minimum_cost" => Ok(Self::MinimumCost),
            "average_target" => Ok(Self::AverageTarget),
            "maximum_target" => Ok(Self::MaximumTarget),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdCampaignBidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MinimumCost => write!(f, "minimum_cost"),
            Self::AverageTarget => write!(f, "average_target"),
            Self::MaximumTarget => write!(f, "maximum_target"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
