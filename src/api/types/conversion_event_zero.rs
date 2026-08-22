pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversionEventZero {
    Purchase,
    AddToCart,
    InitiatedCheckout,
    AddPaymentInfo,
    CompleteRegistration,
    Lead,
    ContentView,
    Search,
    Contact,
    CustomizeProduct,
    Donate,
    FindLocation,
    Schedule,
    StartTrial,
    SubmitApplication,
    Subscribe,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversionEventZero {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::AddToCart => serializer.serialize_str("add_to_cart"),
            Self::InitiatedCheckout => serializer.serialize_str("initiated_checkout"),
            Self::AddPaymentInfo => serializer.serialize_str("add_payment_info"),
            Self::CompleteRegistration => serializer.serialize_str("complete_registration"),
            Self::Lead => serializer.serialize_str("lead"),
            Self::ContentView => serializer.serialize_str("content_view"),
            Self::Search => serializer.serialize_str("search"),
            Self::Contact => serializer.serialize_str("contact"),
            Self::CustomizeProduct => serializer.serialize_str("customize_product"),
            Self::Donate => serializer.serialize_str("donate"),
            Self::FindLocation => serializer.serialize_str("find_location"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::StartTrial => serializer.serialize_str("start_trial"),
            Self::SubmitApplication => serializer.serialize_str("submit_application"),
            Self::Subscribe => serializer.serialize_str("subscribe"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversionEventZero {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "purchase" => Ok(Self::Purchase),
            "add_to_cart" => Ok(Self::AddToCart),
            "initiated_checkout" => Ok(Self::InitiatedCheckout),
            "add_payment_info" => Ok(Self::AddPaymentInfo),
            "complete_registration" => Ok(Self::CompleteRegistration),
            "lead" => Ok(Self::Lead),
            "content_view" => Ok(Self::ContentView),
            "search" => Ok(Self::Search),
            "contact" => Ok(Self::Contact),
            "customize_product" => Ok(Self::CustomizeProduct),
            "donate" => Ok(Self::Donate),
            "find_location" => Ok(Self::FindLocation),
            "schedule" => Ok(Self::Schedule),
            "start_trial" => Ok(Self::StartTrial),
            "submit_application" => Ok(Self::SubmitApplication),
            "subscribe" => Ok(Self::Subscribe),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversionEventZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Purchase => write!(f, "purchase"),
            Self::AddToCart => write!(f, "add_to_cart"),
            Self::InitiatedCheckout => write!(f, "initiated_checkout"),
            Self::AddPaymentInfo => write!(f, "add_payment_info"),
            Self::CompleteRegistration => write!(f, "complete_registration"),
            Self::Lead => write!(f, "lead"),
            Self::ContentView => write!(f, "content_view"),
            Self::Search => write!(f, "search"),
            Self::Contact => write!(f, "contact"),
            Self::CustomizeProduct => write!(f, "customize_product"),
            Self::Donate => write!(f, "donate"),
            Self::FindLocation => write!(f, "find_location"),
            Self::Schedule => write!(f, "schedule"),
            Self::StartTrial => write!(f, "start_trial"),
            Self::SubmitApplication => write!(f, "submit_application"),
            Self::Subscribe => write!(f, "subscribe"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
