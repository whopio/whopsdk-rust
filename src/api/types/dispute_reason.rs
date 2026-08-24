pub use crate::prelude::*;

/// Why the customer says they are disputing, normalized across card networks. `other` covers a code Whop has not categorized yet — read `reason_code` for the raw value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DisputeReason {
    Fraudulent,
    Unrecognized,
    DeclinedAuthorization,
    ProductNotReceived,
    ProductUnacceptable,
    SubscriptionCanceled,
    CreditNotProcessed,
    Duplicate,
    ProcessingError,
    DocumentationRequest,
    BankCannotProcess,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DisputeReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fraudulent => serializer.serialize_str("fraudulent"),
            Self::Unrecognized => serializer.serialize_str("unrecognized"),
            Self::DeclinedAuthorization => serializer.serialize_str("declined_authorization"),
            Self::ProductNotReceived => serializer.serialize_str("product_not_received"),
            Self::ProductUnacceptable => serializer.serialize_str("product_unacceptable"),
            Self::SubscriptionCanceled => serializer.serialize_str("subscription_canceled"),
            Self::CreditNotProcessed => serializer.serialize_str("credit_not_processed"),
            Self::Duplicate => serializer.serialize_str("duplicate"),
            Self::ProcessingError => serializer.serialize_str("processing_error"),
            Self::DocumentationRequest => serializer.serialize_str("documentation_request"),
            Self::BankCannotProcess => serializer.serialize_str("bank_cannot_process"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DisputeReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fraudulent" => Ok(Self::Fraudulent),
            "unrecognized" => Ok(Self::Unrecognized),
            "declined_authorization" => Ok(Self::DeclinedAuthorization),
            "product_not_received" => Ok(Self::ProductNotReceived),
            "product_unacceptable" => Ok(Self::ProductUnacceptable),
            "subscription_canceled" => Ok(Self::SubscriptionCanceled),
            "credit_not_processed" => Ok(Self::CreditNotProcessed),
            "duplicate" => Ok(Self::Duplicate),
            "processing_error" => Ok(Self::ProcessingError),
            "documentation_request" => Ok(Self::DocumentationRequest),
            "bank_cannot_process" => Ok(Self::BankCannotProcess),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DisputeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fraudulent => write!(f, "fraudulent"),
            Self::Unrecognized => write!(f, "unrecognized"),
            Self::DeclinedAuthorization => write!(f, "declined_authorization"),
            Self::ProductNotReceived => write!(f, "product_not_received"),
            Self::ProductUnacceptable => write!(f, "product_unacceptable"),
            Self::SubscriptionCanceled => write!(f, "subscription_canceled"),
            Self::CreditNotProcessed => write!(f, "credit_not_processed"),
            Self::Duplicate => write!(f, "duplicate"),
            Self::ProcessingError => write!(f, "processing_error"),
            Self::DocumentationRequest => write!(f, "documentation_request"),
            Self::BankCannotProcess => write!(f, "bank_cannot_process"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
