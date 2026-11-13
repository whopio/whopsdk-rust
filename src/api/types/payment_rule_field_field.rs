pub use crate::prelude::*;

/// The payment attribute this condition reads.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleFieldField {
    RiskScore,
    AmountInUsd,
    CardCountry,
    CardBrand,
    CardFunding,
    CardBin,
    BillingCountry,
    BillingPostalCode,
    BillingCity,
    CustomerEmail,
    EmailDomain,
    IpAddress,
    IpCountry,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleFieldField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RiskScore => serializer.serialize_str("risk_score"),
            Self::AmountInUsd => serializer.serialize_str("amount_in_usd"),
            Self::CardCountry => serializer.serialize_str("card_country"),
            Self::CardBrand => serializer.serialize_str("card_brand"),
            Self::CardFunding => serializer.serialize_str("card_funding"),
            Self::CardBin => serializer.serialize_str("card_bin"),
            Self::BillingCountry => serializer.serialize_str("billing_country"),
            Self::BillingPostalCode => serializer.serialize_str("billing_postal_code"),
            Self::BillingCity => serializer.serialize_str("billing_city"),
            Self::CustomerEmail => serializer.serialize_str("customer_email"),
            Self::EmailDomain => serializer.serialize_str("email_domain"),
            Self::IpAddress => serializer.serialize_str("ip_address"),
            Self::IpCountry => serializer.serialize_str("ip_country"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleFieldField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "risk_score" => Ok(Self::RiskScore),
            "amount_in_usd" => Ok(Self::AmountInUsd),
            "card_country" => Ok(Self::CardCountry),
            "card_brand" => Ok(Self::CardBrand),
            "card_funding" => Ok(Self::CardFunding),
            "card_bin" => Ok(Self::CardBin),
            "billing_country" => Ok(Self::BillingCountry),
            "billing_postal_code" => Ok(Self::BillingPostalCode),
            "billing_city" => Ok(Self::BillingCity),
            "customer_email" => Ok(Self::CustomerEmail),
            "email_domain" => Ok(Self::EmailDomain),
            "ip_address" => Ok(Self::IpAddress),
            "ip_country" => Ok(Self::IpCountry),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleFieldField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RiskScore => write!(f, "risk_score"),
            Self::AmountInUsd => write!(f, "amount_in_usd"),
            Self::CardCountry => write!(f, "card_country"),
            Self::CardBrand => write!(f, "card_brand"),
            Self::CardFunding => write!(f, "card_funding"),
            Self::CardBin => write!(f, "card_bin"),
            Self::BillingCountry => write!(f, "billing_country"),
            Self::BillingPostalCode => write!(f, "billing_postal_code"),
            Self::BillingCity => write!(f, "billing_city"),
            Self::CustomerEmail => write!(f, "customer_email"),
            Self::EmailDomain => write!(f, "email_domain"),
            Self::IpAddress => write!(f, "ip_address"),
            Self::IpCountry => write!(f, "ip_country"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
