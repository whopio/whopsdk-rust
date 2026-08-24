pub use crate::prelude::*;

/// Regulated categories the campaign is declared under. Ads in these categories are subject to extra targeting restrictions. Empty when none apply.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdCampaignSpecialAdCategoriesItem {
    Housing,
    Employment,
    FinancialProducts,
    Politics,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdCampaignSpecialAdCategoriesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Housing => serializer.serialize_str("housing"),
            Self::Employment => serializer.serialize_str("employment"),
            Self::FinancialProducts => serializer.serialize_str("financial_products"),
            Self::Politics => serializer.serialize_str("politics"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdCampaignSpecialAdCategoriesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "housing" => Ok(Self::Housing),
            "employment" => Ok(Self::Employment),
            "financial_products" => Ok(Self::FinancialProducts),
            "politics" => Ok(Self::Politics),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdCampaignSpecialAdCategoriesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Housing => write!(f, "housing"),
            Self::Employment => write!(f, "employment"),
            Self::FinancialProducts => write!(f, "financial_products"),
            Self::Politics => write!(f, "politics"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
