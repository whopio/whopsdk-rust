pub use crate::prelude::*;

/// What the cardholder claims. `fraud` when they say they didn't make the payment: always for an `early_fraud_warning`, and for Ethoca fraud alerts and Visa RDR cases filed under a fraud reason. `customer_dispute` when they made the payment but dispute it, such as a product not received or a canceled subscription. `null` when the alert carries no claim, such as an issuer's request for transaction documents.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeAlertCategory {
    Fraud,
    CustomerDispute,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeAlertCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fraud => serializer.serialize_str("fraud"),
            Self::CustomerDispute => serializer.serialize_str("customer_dispute"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeAlertCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fraud" => Ok(Self::Fraud),
            "customer_dispute" => Ok(Self::CustomerDispute),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeAlertCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fraud => write!(f, "fraud"),
            Self::CustomerDispute => write!(f, "customer_dispute"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
