pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAdCampaignsRequestOrder {
    CreatedAt,
    UpdatedAt,
    Spend,
    Impressions,
    Reach,
    Clicks,
    LinkClicks,
    UniqueClicks,
    Frequency,
    ClickThroughRate,
    Results,
    CostPerMille,
    CostPerClick,
    CostPerResult,
    ReturnOnAdSpend,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAdCampaignsRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::UpdatedAt => serializer.serialize_str("updated_at"),
            Self::Spend => serializer.serialize_str("spend"),
            Self::Impressions => serializer.serialize_str("impressions"),
            Self::Reach => serializer.serialize_str("reach"),
            Self::Clicks => serializer.serialize_str("clicks"),
            Self::LinkClicks => serializer.serialize_str("link_clicks"),
            Self::UniqueClicks => serializer.serialize_str("unique_clicks"),
            Self::Frequency => serializer.serialize_str("frequency"),
            Self::ClickThroughRate => serializer.serialize_str("click_through_rate"),
            Self::Results => serializer.serialize_str("results"),
            Self::CostPerMille => serializer.serialize_str("cost_per_mille"),
            Self::CostPerClick => serializer.serialize_str("cost_per_click"),
            Self::CostPerResult => serializer.serialize_str("cost_per_result"),
            Self::ReturnOnAdSpend => serializer.serialize_str("return_on_ad_spend"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAdCampaignsRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            "spend" => Ok(Self::Spend),
            "impressions" => Ok(Self::Impressions),
            "reach" => Ok(Self::Reach),
            "clicks" => Ok(Self::Clicks),
            "link_clicks" => Ok(Self::LinkClicks),
            "unique_clicks" => Ok(Self::UniqueClicks),
            "frequency" => Ok(Self::Frequency),
            "click_through_rate" => Ok(Self::ClickThroughRate),
            "results" => Ok(Self::Results),
            "cost_per_mille" => Ok(Self::CostPerMille),
            "cost_per_click" => Ok(Self::CostPerClick),
            "cost_per_result" => Ok(Self::CostPerResult),
            "return_on_ad_spend" => Ok(Self::ReturnOnAdSpend),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAdCampaignsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::Spend => write!(f, "spend"),
            Self::Impressions => write!(f, "impressions"),
            Self::Reach => write!(f, "reach"),
            Self::Clicks => write!(f, "clicks"),
            Self::LinkClicks => write!(f, "link_clicks"),
            Self::UniqueClicks => write!(f, "unique_clicks"),
            Self::Frequency => write!(f, "frequency"),
            Self::ClickThroughRate => write!(f, "click_through_rate"),
            Self::Results => write!(f, "results"),
            Self::CostPerMille => write!(f, "cost_per_mille"),
            Self::CostPerClick => write!(f, "cost_per_click"),
            Self::CostPerResult => write!(f, "cost_per_result"),
            Self::ReturnOnAdSpend => write!(f, "return_on_ad_spend"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
