pub use crate::prelude::*;

/// What changes the price: the promo's `discount`, the `buyer_fee` the charge adds, or `tax`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownAdjustmentKind {
    Discount,
    BuyerFee,
    Tax,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBreakdownAdjustmentKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Discount => serializer.serialize_str("discount"),
            Self::BuyerFee => serializer.serialize_str("buyer_fee"),
            Self::Tax => serializer.serialize_str("tax"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBreakdownAdjustmentKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "discount" => Ok(Self::Discount),
            "buyer_fee" => Ok(Self::BuyerFee),
            "tax" => Ok(Self::Tax),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBreakdownAdjustmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discount => write!(f, "discount"),
            Self::BuyerFee => write!(f, "buyer_fee"),
            Self::Tax => write!(f, "tax"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
