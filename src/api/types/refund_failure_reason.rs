pub use crate::prelude::*;

/// Why the refund failed, normalized across providers. Null unless the refund failed or was canceled.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefundFailureReason {
    BankDeclined,
    ExpiredOrCanceledCard,
    LostOrStolenCard,
    InsufficientFunds,
    ChargeDisputed,
    NotRefundable,
    MerchantRequest,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RefundFailureReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BankDeclined => serializer.serialize_str("bank_declined"),
            Self::ExpiredOrCanceledCard => serializer.serialize_str("expired_or_canceled_card"),
            Self::LostOrStolenCard => serializer.serialize_str("lost_or_stolen_card"),
            Self::InsufficientFunds => serializer.serialize_str("insufficient_funds"),
            Self::ChargeDisputed => serializer.serialize_str("charge_disputed"),
            Self::NotRefundable => serializer.serialize_str("not_refundable"),
            Self::MerchantRequest => serializer.serialize_str("merchant_request"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RefundFailureReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bank_declined" => Ok(Self::BankDeclined),
            "expired_or_canceled_card" => Ok(Self::ExpiredOrCanceledCard),
            "lost_or_stolen_card" => Ok(Self::LostOrStolenCard),
            "insufficient_funds" => Ok(Self::InsufficientFunds),
            "charge_disputed" => Ok(Self::ChargeDisputed),
            "not_refundable" => Ok(Self::NotRefundable),
            "merchant_request" => Ok(Self::MerchantRequest),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RefundFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BankDeclined => write!(f, "bank_declined"),
            Self::ExpiredOrCanceledCard => write!(f, "expired_or_canceled_card"),
            Self::LostOrStolenCard => write!(f, "lost_or_stolen_card"),
            Self::InsufficientFunds => write!(f, "insufficient_funds"),
            Self::ChargeDisputed => write!(f, "charge_disputed"),
            Self::NotRefundable => write!(f, "not_refundable"),
            Self::MerchantRequest => write!(f, "merchant_request"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
