pub use crate::prelude::*;

/// The payment attribute this condition reads.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePaymentRulesRequestConditionsAllItemField {
    RiskScore,
    CardCountry,
    CustomerEmail,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePaymentRulesRequestConditionsAllItemField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RiskScore => serializer.serialize_str("risk_score"),
            Self::CardCountry => serializer.serialize_str("card_country"),
            Self::CustomerEmail => serializer.serialize_str("customer_email"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePaymentRulesRequestConditionsAllItemField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "risk_score" => Ok(Self::RiskScore),
            "card_country" => Ok(Self::CardCountry),
            "customer_email" => Ok(Self::CustomerEmail),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePaymentRulesRequestConditionsAllItemField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RiskScore => write!(f, "risk_score"),
            Self::CardCountry => write!(f, "card_country"),
            Self::CustomerEmail => write!(f, "customer_email"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
