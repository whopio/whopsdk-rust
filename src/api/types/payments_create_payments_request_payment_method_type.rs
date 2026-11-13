pub use crate::prelude::*;

/// The kind of payment method; the member named after it carries its details.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePaymentsRequestPaymentMethodType {
    CardPresent,
    Paypal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePaymentsRequestPaymentMethodType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CardPresent => serializer.serialize_str("card_present"),
            Self::Paypal => serializer.serialize_str("paypal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePaymentsRequestPaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card_present" => Ok(Self::CardPresent),
            "paypal" => Ok(Self::Paypal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePaymentsRequestPaymentMethodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CardPresent => write!(f, "card_present"),
            Self::Paypal => write!(f, "paypal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
