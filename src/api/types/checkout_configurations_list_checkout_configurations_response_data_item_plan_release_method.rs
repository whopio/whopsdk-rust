pub use crate::prelude::*;

/// Sales method for the plan.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod {
    BuyNow,
    Waitlist,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BuyNow => serializer.serialize_str("buy_now"),
            Self::Waitlist => serializer.serialize_str("waitlist"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "buy_now" => Ok(Self::BuyNow),
            "waitlist" => Ok(Self::Waitlist),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCheckoutConfigurationsResponseDataItemPlanReleaseMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuyNow => write!(f, "buy_now"),
            Self::Waitlist => write!(f, "waitlist"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
