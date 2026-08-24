pub use crate::prelude::*;

/// The call-to-action button label.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateProductsRequestCustomCta {
    GetAccess,
    Join,
    OrderNow,
    ShopNow,
    CallNow,
    DonateNow,
    ContactUs,
    SignUp,
    Subscribe,
    Purchase,
    GetOffer,
    ApplyNow,
    CompleteOrder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateProductsRequestCustomCta {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GetAccess => serializer.serialize_str("get_access"),
            Self::Join => serializer.serialize_str("join"),
            Self::OrderNow => serializer.serialize_str("order_now"),
            Self::ShopNow => serializer.serialize_str("shop_now"),
            Self::CallNow => serializer.serialize_str("call_now"),
            Self::DonateNow => serializer.serialize_str("donate_now"),
            Self::ContactUs => serializer.serialize_str("contact_us"),
            Self::SignUp => serializer.serialize_str("sign_up"),
            Self::Subscribe => serializer.serialize_str("subscribe"),
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::GetOffer => serializer.serialize_str("get_offer"),
            Self::ApplyNow => serializer.serialize_str("apply_now"),
            Self::CompleteOrder => serializer.serialize_str("complete_order"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateProductsRequestCustomCta {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "get_access" => Ok(Self::GetAccess),
            "join" => Ok(Self::Join),
            "order_now" => Ok(Self::OrderNow),
            "shop_now" => Ok(Self::ShopNow),
            "call_now" => Ok(Self::CallNow),
            "donate_now" => Ok(Self::DonateNow),
            "contact_us" => Ok(Self::ContactUs),
            "sign_up" => Ok(Self::SignUp),
            "subscribe" => Ok(Self::Subscribe),
            "purchase" => Ok(Self::Purchase),
            "get_offer" => Ok(Self::GetOffer),
            "apply_now" => Ok(Self::ApplyNow),
            "complete_order" => Ok(Self::CompleteOrder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateProductsRequestCustomCta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GetAccess => write!(f, "get_access"),
            Self::Join => write!(f, "join"),
            Self::OrderNow => write!(f, "order_now"),
            Self::ShopNow => write!(f, "shop_now"),
            Self::CallNow => write!(f, "call_now"),
            Self::DonateNow => write!(f, "donate_now"),
            Self::ContactUs => write!(f, "contact_us"),
            Self::SignUp => write!(f, "sign_up"),
            Self::Subscribe => write!(f, "subscribe"),
            Self::Purchase => write!(f, "purchase"),
            Self::GetOffer => write!(f, "get_offer"),
            Self::ApplyNow => write!(f, "apply_now"),
            Self::CompleteOrder => write!(f, "complete_order"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
