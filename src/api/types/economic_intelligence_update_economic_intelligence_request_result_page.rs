pub use crate::prelude::*;

/// With `status: executed`, the page where the run's result can be seen when it is not one resource, such as the checkout links list. The recommendation's `result_url` becomes that page on the account's dashboard.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateEconomicIntelligenceRequestResultPage {
    Home,
    Products,
    Ads,
    Websites,
    CheckoutLinks,
    TrackingLinks,
    PromoCodes,
    Payments,
    Customers,
    Affiliates,
    Analytics,
    StorePage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateEconomicIntelligenceRequestResultPage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Home => serializer.serialize_str("home"),
            Self::Products => serializer.serialize_str("products"),
            Self::Ads => serializer.serialize_str("ads"),
            Self::Websites => serializer.serialize_str("websites"),
            Self::CheckoutLinks => serializer.serialize_str("checkout_links"),
            Self::TrackingLinks => serializer.serialize_str("tracking_links"),
            Self::PromoCodes => serializer.serialize_str("promo_codes"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::Customers => serializer.serialize_str("customers"),
            Self::Affiliates => serializer.serialize_str("affiliates"),
            Self::Analytics => serializer.serialize_str("analytics"),
            Self::StorePage => serializer.serialize_str("store_page"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateEconomicIntelligenceRequestResultPage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "home" => Ok(Self::Home),
            "products" => Ok(Self::Products),
            "ads" => Ok(Self::Ads),
            "websites" => Ok(Self::Websites),
            "checkout_links" => Ok(Self::CheckoutLinks),
            "tracking_links" => Ok(Self::TrackingLinks),
            "promo_codes" => Ok(Self::PromoCodes),
            "payments" => Ok(Self::Payments),
            "customers" => Ok(Self::Customers),
            "affiliates" => Ok(Self::Affiliates),
            "analytics" => Ok(Self::Analytics),
            "store_page" => Ok(Self::StorePage),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateEconomicIntelligenceRequestResultPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Home => write!(f, "home"),
            Self::Products => write!(f, "products"),
            Self::Ads => write!(f, "ads"),
            Self::Websites => write!(f, "websites"),
            Self::CheckoutLinks => write!(f, "checkout_links"),
            Self::TrackingLinks => write!(f, "tracking_links"),
            Self::PromoCodes => write!(f, "promo_codes"),
            Self::Payments => write!(f, "payments"),
            Self::Customers => write!(f, "customers"),
            Self::Affiliates => write!(f, "affiliates"),
            Self::Analytics => write!(f, "analytics"),
            Self::StorePage => write!(f, "store_page"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
