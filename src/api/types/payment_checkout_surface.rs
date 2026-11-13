pub use crate::prelude::*;

/// The checkout experience that produced this payment, such as `embedded_checkout` for a checkout on the account's own website or `off_session` for a subscription renewal. Null when the checkout was not recorded.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentCheckoutSurface {
    HostedCheckout,
    EmbeddedCheckout,
    CustomCheckout,
    ModalCheckout,
    InAppCheckout,
    MobileCheckout,
    OffSession,
    CardPresent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentCheckoutSurface {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::HostedCheckout => serializer.serialize_str("hosted_checkout"),
            Self::EmbeddedCheckout => serializer.serialize_str("embedded_checkout"),
            Self::CustomCheckout => serializer.serialize_str("custom_checkout"),
            Self::ModalCheckout => serializer.serialize_str("modal_checkout"),
            Self::InAppCheckout => serializer.serialize_str("in_app_checkout"),
            Self::MobileCheckout => serializer.serialize_str("mobile_checkout"),
            Self::OffSession => serializer.serialize_str("off_session"),
            Self::CardPresent => serializer.serialize_str("card_present"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentCheckoutSurface {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "hosted_checkout" => Ok(Self::HostedCheckout),
            "embedded_checkout" => Ok(Self::EmbeddedCheckout),
            "custom_checkout" => Ok(Self::CustomCheckout),
            "modal_checkout" => Ok(Self::ModalCheckout),
            "in_app_checkout" => Ok(Self::InAppCheckout),
            "mobile_checkout" => Ok(Self::MobileCheckout),
            "off_session" => Ok(Self::OffSession),
            "card_present" => Ok(Self::CardPresent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentCheckoutSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HostedCheckout => write!(f, "hosted_checkout"),
            Self::EmbeddedCheckout => write!(f, "embedded_checkout"),
            Self::CustomCheckout => write!(f, "custom_checkout"),
            Self::ModalCheckout => write!(f, "modal_checkout"),
            Self::InAppCheckout => write!(f, "in_app_checkout"),
            Self::MobileCheckout => write!(f, "mobile_checkout"),
            Self::OffSession => write!(f, "off_session"),
            Self::CardPresent => write!(f, "card_present"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
