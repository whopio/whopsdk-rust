pub use crate::prelude::*;

/// The payment attribute this condition reads.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleConditionField {
    RiskScore,
    AmountInUsd,
    CardCountry,
    CustomerEmail,
    IpAddress,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleConditionField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RiskScore => serializer.serialize_str("risk_score"),
            Self::AmountInUsd => serializer.serialize_str("amount_in_usd"),
            Self::CardCountry => serializer.serialize_str("card_country"),
            Self::CustomerEmail => serializer.serialize_str("customer_email"),
            Self::IpAddress => serializer.serialize_str("ip_address"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleConditionField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "risk_score" => Ok(Self::RiskScore),
            "amount_in_usd" => Ok(Self::AmountInUsd),
            "card_country" => Ok(Self::CardCountry),
            "customer_email" => Ok(Self::CustomerEmail),
            "ip_address" => Ok(Self::IpAddress),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleConditionField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RiskScore => write!(f, "risk_score"),
            Self::AmountInUsd => write!(f, "amount_in_usd"),
            Self::CardCountry => write!(f, "card_country"),
            Self::CustomerEmail => write!(f, "customer_email"),
            Self::IpAddress => write!(f, "ip_address"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
