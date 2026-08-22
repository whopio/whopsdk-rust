pub use crate::prelude::*;

/// The different types of payment transactions.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentTransactionTypes {
    Purchase,
    Authorize,
    Capture,
    Refund,
    Canceled,
    Verify,
    Chargeback,
    PreChargeback,
    ThreeDSecure,
    FraudScreening,
    Authorization,
    Installment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentTransactionTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::Authorize => serializer.serialize_str("authorize"),
            Self::Capture => serializer.serialize_str("capture"),
            Self::Refund => serializer.serialize_str("refund"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Verify => serializer.serialize_str("verify"),
            Self::Chargeback => serializer.serialize_str("chargeback"),
            Self::PreChargeback => serializer.serialize_str("pre_chargeback"),
            Self::ThreeDSecure => serializer.serialize_str("three_d_secure"),
            Self::FraudScreening => serializer.serialize_str("fraud_screening"),
            Self::Authorization => serializer.serialize_str("authorization"),
            Self::Installment => serializer.serialize_str("installment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentTransactionTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "purchase" => Ok(Self::Purchase),
            "authorize" => Ok(Self::Authorize),
            "capture" => Ok(Self::Capture),
            "refund" => Ok(Self::Refund),
            "canceled" => Ok(Self::Canceled),
            "verify" => Ok(Self::Verify),
            "chargeback" => Ok(Self::Chargeback),
            "pre_chargeback" => Ok(Self::PreChargeback),
            "three_d_secure" => Ok(Self::ThreeDSecure),
            "fraud_screening" => Ok(Self::FraudScreening),
            "authorization" => Ok(Self::Authorization),
            "installment" => Ok(Self::Installment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentTransactionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Purchase => write!(f, "purchase"),
            Self::Authorize => write!(f, "authorize"),
            Self::Capture => write!(f, "capture"),
            Self::Refund => write!(f, "refund"),
            Self::Canceled => write!(f, "canceled"),
            Self::Verify => write!(f, "verify"),
            Self::Chargeback => write!(f, "chargeback"),
            Self::PreChargeback => write!(f, "pre_chargeback"),
            Self::ThreeDSecure => write!(f, "three_d_secure"),
            Self::FraudScreening => write!(f, "fraud_screening"),
            Self::Authorization => write!(f, "authorization"),
            Self::Installment => write!(f, "installment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
