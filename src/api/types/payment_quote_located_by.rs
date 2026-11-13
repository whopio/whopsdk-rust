pub use crate::prelude::*;

/// Which location tax was calculated for: `shipping_address` when it carries a country, else the billing `address` when it does, else the `presentment_country` sent, else the buyer's `ip_address`. A quote located by `presentment_country` or `ip_address` is a preview that estimates tax: a payment cannot use it, so quote again with the buyer's address to pay. Null when nothing in the request located the buyer, which only a seller that collects no tax on this purchase is quoted without; `tax_status` is then `not_applicable`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentQuoteLocatedBy {
    ShippingAddress,
    Address,
    PresentmentCountry,
    IpAddress,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentQuoteLocatedBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ShippingAddress => serializer.serialize_str("shipping_address"),
            Self::Address => serializer.serialize_str("address"),
            Self::PresentmentCountry => serializer.serialize_str("presentment_country"),
            Self::IpAddress => serializer.serialize_str("ip_address"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentQuoteLocatedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "shipping_address" => Ok(Self::ShippingAddress),
            "address" => Ok(Self::Address),
            "presentment_country" => Ok(Self::PresentmentCountry),
            "ip_address" => Ok(Self::IpAddress),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentQuoteLocatedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShippingAddress => write!(f, "shipping_address"),
            Self::Address => write!(f, "address"),
            Self::PresentmentCountry => write!(f, "presentment_country"),
            Self::IpAddress => write!(f, "ip_address"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
