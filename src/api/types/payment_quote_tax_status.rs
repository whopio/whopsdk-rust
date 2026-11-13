pub use crate::prelude::*;

/// `calculated`: every line was priced and a payment may consume the quote. `not_applicable`: this seller collects no tax on this purchase, so the quote owes none and may still be consumed. `unavailable`: tax could not be priced — the provider did not answer, or this seller's tax setup cannot price a purchase here — so a payment refuses the quote; quote again, or charge without `quote_id` to have tax calculated at charge time.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentQuoteTaxStatus {
    Calculated,
    NotApplicable,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentQuoteTaxStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Calculated => serializer.serialize_str("calculated"),
            Self::NotApplicable => serializer.serialize_str("not_applicable"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentQuoteTaxStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "calculated" => Ok(Self::Calculated),
            "not_applicable" => Ok(Self::NotApplicable),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentQuoteTaxStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Calculated => write!(f, "calculated"),
            Self::NotApplicable => write!(f, "not_applicable"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
