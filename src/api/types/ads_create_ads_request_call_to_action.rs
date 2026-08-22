pub use crate::prelude::*;

/// The call-to-action button shown on the ad.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAdsRequestCallToAction {
    ApplyNow,
    BookNow,
    CallNow,
    ContactUs,
    Download,
    GetDirections,
    GetOffer,
    GetQuote,
    LearnMore,
    ListenNow,
    MessagePage,
    NoButton,
    OpenLink,
    OrderNow,
    RequestTime,
    SeeDetails,
    SeeMenu,
    SendUpdates,
    ShopNow,
    SignUp,
    Subscribe,
    WatchMore,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAdsRequestCallToAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ApplyNow => serializer.serialize_str("apply_now"),
            Self::BookNow => serializer.serialize_str("book_now"),
            Self::CallNow => serializer.serialize_str("call_now"),
            Self::ContactUs => serializer.serialize_str("contact_us"),
            Self::Download => serializer.serialize_str("download"),
            Self::GetDirections => serializer.serialize_str("get_directions"),
            Self::GetOffer => serializer.serialize_str("get_offer"),
            Self::GetQuote => serializer.serialize_str("get_quote"),
            Self::LearnMore => serializer.serialize_str("learn_more"),
            Self::ListenNow => serializer.serialize_str("listen_now"),
            Self::MessagePage => serializer.serialize_str("message_page"),
            Self::NoButton => serializer.serialize_str("no_button"),
            Self::OpenLink => serializer.serialize_str("open_link"),
            Self::OrderNow => serializer.serialize_str("order_now"),
            Self::RequestTime => serializer.serialize_str("request_time"),
            Self::SeeDetails => serializer.serialize_str("see_details"),
            Self::SeeMenu => serializer.serialize_str("see_menu"),
            Self::SendUpdates => serializer.serialize_str("send_updates"),
            Self::ShopNow => serializer.serialize_str("shop_now"),
            Self::SignUp => serializer.serialize_str("sign_up"),
            Self::Subscribe => serializer.serialize_str("subscribe"),
            Self::WatchMore => serializer.serialize_str("watch_more"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAdsRequestCallToAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "apply_now" => Ok(Self::ApplyNow),
            "book_now" => Ok(Self::BookNow),
            "call_now" => Ok(Self::CallNow),
            "contact_us" => Ok(Self::ContactUs),
            "download" => Ok(Self::Download),
            "get_directions" => Ok(Self::GetDirections),
            "get_offer" => Ok(Self::GetOffer),
            "get_quote" => Ok(Self::GetQuote),
            "learn_more" => Ok(Self::LearnMore),
            "listen_now" => Ok(Self::ListenNow),
            "message_page" => Ok(Self::MessagePage),
            "no_button" => Ok(Self::NoButton),
            "open_link" => Ok(Self::OpenLink),
            "order_now" => Ok(Self::OrderNow),
            "request_time" => Ok(Self::RequestTime),
            "see_details" => Ok(Self::SeeDetails),
            "see_menu" => Ok(Self::SeeMenu),
            "send_updates" => Ok(Self::SendUpdates),
            "shop_now" => Ok(Self::ShopNow),
            "sign_up" => Ok(Self::SignUp),
            "subscribe" => Ok(Self::Subscribe),
            "watch_more" => Ok(Self::WatchMore),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAdsRequestCallToAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApplyNow => write!(f, "apply_now"),
            Self::BookNow => write!(f, "book_now"),
            Self::CallNow => write!(f, "call_now"),
            Self::ContactUs => write!(f, "contact_us"),
            Self::Download => write!(f, "download"),
            Self::GetDirections => write!(f, "get_directions"),
            Self::GetOffer => write!(f, "get_offer"),
            Self::GetQuote => write!(f, "get_quote"),
            Self::LearnMore => write!(f, "learn_more"),
            Self::ListenNow => write!(f, "listen_now"),
            Self::MessagePage => write!(f, "message_page"),
            Self::NoButton => write!(f, "no_button"),
            Self::OpenLink => write!(f, "open_link"),
            Self::OrderNow => write!(f, "order_now"),
            Self::RequestTime => write!(f, "request_time"),
            Self::SeeDetails => write!(f, "see_details"),
            Self::SeeMenu => write!(f, "see_menu"),
            Self::SendUpdates => write!(f, "send_updates"),
            Self::ShopNow => write!(f, "shop_now"),
            Self::SignUp => write!(f, "sign_up"),
            Self::Subscribe => write!(f, "subscribe"),
            Self::WatchMore => write!(f, "watch_more"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
