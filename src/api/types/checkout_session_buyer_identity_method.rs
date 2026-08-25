pub use crate::prelude::*;

/// How the buyer's identity was established — the mechanism behind `buyer_identity`, so a checkout stays explicable long after it completed. `null` before a buyer is resolved. New mechanisms are added over time.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBuyerIdentityMethod {
    ConfirmationTokenEmail,
    BuyerEmail,
    SavedPaymentMethod,
    SessionIntent,
    PostPurchaseClaim,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBuyerIdentityMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConfirmationTokenEmail => serializer.serialize_str("confirmation_token_email"),
            Self::BuyerEmail => serializer.serialize_str("buyer_email"),
            Self::SavedPaymentMethod => serializer.serialize_str("saved_payment_method"),
            Self::SessionIntent => serializer.serialize_str("session_intent"),
            Self::PostPurchaseClaim => serializer.serialize_str("post_purchase_claim"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBuyerIdentityMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "confirmation_token_email" => Ok(Self::ConfirmationTokenEmail),
            "buyer_email" => Ok(Self::BuyerEmail),
            "saved_payment_method" => Ok(Self::SavedPaymentMethod),
            "session_intent" => Ok(Self::SessionIntent),
            "post_purchase_claim" => Ok(Self::PostPurchaseClaim),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBuyerIdentityMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfirmationTokenEmail => write!(f, "confirmation_token_email"),
            Self::BuyerEmail => write!(f, "buyer_email"),
            Self::SavedPaymentMethod => write!(f, "saved_payment_method"),
            Self::SessionIntent => write!(f, "session_intent"),
            Self::PostPurchaseClaim => write!(f, "post_purchase_claim"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
