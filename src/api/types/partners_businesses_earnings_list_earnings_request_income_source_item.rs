pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEarningsRequestIncomeSourceItem {
    Sales,
    AdSpend,
    Transfer,
    CardInterchange,
    OnboardingReward,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEarningsRequestIncomeSourceItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sales => serializer.serialize_str("sales"),
            Self::AdSpend => serializer.serialize_str("ad_spend"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::CardInterchange => serializer.serialize_str("card_interchange"),
            Self::OnboardingReward => serializer.serialize_str("onboarding_reward"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEarningsRequestIncomeSourceItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sales" => Ok(Self::Sales),
            "ad_spend" => Ok(Self::AdSpend),
            "transfer" => Ok(Self::Transfer),
            "card_interchange" => Ok(Self::CardInterchange),
            "onboarding_reward" => Ok(Self::OnboardingReward),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEarningsRequestIncomeSourceItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sales => write!(f, "sales"),
            Self::AdSpend => write!(f, "ad_spend"),
            Self::Transfer => write!(f, "transfer"),
            Self::CardInterchange => write!(f, "card_interchange"),
            Self::OnboardingReward => write!(f, "onboarding_reward"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
