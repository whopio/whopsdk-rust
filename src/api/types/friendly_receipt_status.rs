pub use crate::prelude::*;

/// The friendly status of a payment. This is a derived status that provides a human-readable summary of the payment state, combining the underlying status and substatus fields.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FriendlyReceiptStatus {
    Succeeded,
    Pending,
    Failed,
    PastDue,
    Canceled,
    PriceTooLow,
    Uncollectible,
    Refunded,
    AutoRefunded,
    PartiallyRefunded,
    DisputeWarning,
    DisputeNeedsResponse,
    DisputeWarningNeedsResponse,
    ResolutionNeedsResponse,
    DisputeUnderReview,
    DisputeWarningUnderReview,
    ResolutionUnderReview,
    DisputeWon,
    DisputeWarningClosed,
    ResolutionWon,
    DisputeLost,
    DisputeClosed,
    ResolutionLost,
    Drafted,
    Incomplete,
    Unresolved,
    OpenDispute,
    OpenResolution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FriendlyReceiptStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::PriceTooLow => serializer.serialize_str("price_too_low"),
            Self::Uncollectible => serializer.serialize_str("uncollectible"),
            Self::Refunded => serializer.serialize_str("refunded"),
            Self::AutoRefunded => serializer.serialize_str("auto_refunded"),
            Self::PartiallyRefunded => serializer.serialize_str("partially_refunded"),
            Self::DisputeWarning => serializer.serialize_str("dispute_warning"),
            Self::DisputeNeedsResponse => serializer.serialize_str("dispute_needs_response"),
            Self::DisputeWarningNeedsResponse => {
                serializer.serialize_str("dispute_warning_needs_response")
            }
            Self::ResolutionNeedsResponse => serializer.serialize_str("resolution_needs_response"),
            Self::DisputeUnderReview => serializer.serialize_str("dispute_under_review"),
            Self::DisputeWarningUnderReview => {
                serializer.serialize_str("dispute_warning_under_review")
            }
            Self::ResolutionUnderReview => serializer.serialize_str("resolution_under_review"),
            Self::DisputeWon => serializer.serialize_str("dispute_won"),
            Self::DisputeWarningClosed => serializer.serialize_str("dispute_warning_closed"),
            Self::ResolutionWon => serializer.serialize_str("resolution_won"),
            Self::DisputeLost => serializer.serialize_str("dispute_lost"),
            Self::DisputeClosed => serializer.serialize_str("dispute_closed"),
            Self::ResolutionLost => serializer.serialize_str("resolution_lost"),
            Self::Drafted => serializer.serialize_str("drafted"),
            Self::Incomplete => serializer.serialize_str("incomplete"),
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::OpenDispute => serializer.serialize_str("open_dispute"),
            Self::OpenResolution => serializer.serialize_str("open_resolution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FriendlyReceiptStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "succeeded" => Ok(Self::Succeeded),
            "pending" => Ok(Self::Pending),
            "failed" => Ok(Self::Failed),
            "past_due" => Ok(Self::PastDue),
            "canceled" => Ok(Self::Canceled),
            "price_too_low" => Ok(Self::PriceTooLow),
            "uncollectible" => Ok(Self::Uncollectible),
            "refunded" => Ok(Self::Refunded),
            "auto_refunded" => Ok(Self::AutoRefunded),
            "partially_refunded" => Ok(Self::PartiallyRefunded),
            "dispute_warning" => Ok(Self::DisputeWarning),
            "dispute_needs_response" => Ok(Self::DisputeNeedsResponse),
            "dispute_warning_needs_response" => Ok(Self::DisputeWarningNeedsResponse),
            "resolution_needs_response" => Ok(Self::ResolutionNeedsResponse),
            "dispute_under_review" => Ok(Self::DisputeUnderReview),
            "dispute_warning_under_review" => Ok(Self::DisputeWarningUnderReview),
            "resolution_under_review" => Ok(Self::ResolutionUnderReview),
            "dispute_won" => Ok(Self::DisputeWon),
            "dispute_warning_closed" => Ok(Self::DisputeWarningClosed),
            "resolution_won" => Ok(Self::ResolutionWon),
            "dispute_lost" => Ok(Self::DisputeLost),
            "dispute_closed" => Ok(Self::DisputeClosed),
            "resolution_lost" => Ok(Self::ResolutionLost),
            "drafted" => Ok(Self::Drafted),
            "incomplete" => Ok(Self::Incomplete),
            "unresolved" => Ok(Self::Unresolved),
            "open_dispute" => Ok(Self::OpenDispute),
            "open_resolution" => Ok(Self::OpenResolution),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FriendlyReceiptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded => write!(f, "succeeded"),
            Self::Pending => write!(f, "pending"),
            Self::Failed => write!(f, "failed"),
            Self::PastDue => write!(f, "past_due"),
            Self::Canceled => write!(f, "canceled"),
            Self::PriceTooLow => write!(f, "price_too_low"),
            Self::Uncollectible => write!(f, "uncollectible"),
            Self::Refunded => write!(f, "refunded"),
            Self::AutoRefunded => write!(f, "auto_refunded"),
            Self::PartiallyRefunded => write!(f, "partially_refunded"),
            Self::DisputeWarning => write!(f, "dispute_warning"),
            Self::DisputeNeedsResponse => write!(f, "dispute_needs_response"),
            Self::DisputeWarningNeedsResponse => write!(f, "dispute_warning_needs_response"),
            Self::ResolutionNeedsResponse => write!(f, "resolution_needs_response"),
            Self::DisputeUnderReview => write!(f, "dispute_under_review"),
            Self::DisputeWarningUnderReview => write!(f, "dispute_warning_under_review"),
            Self::ResolutionUnderReview => write!(f, "resolution_under_review"),
            Self::DisputeWon => write!(f, "dispute_won"),
            Self::DisputeWarningClosed => write!(f, "dispute_warning_closed"),
            Self::ResolutionWon => write!(f, "resolution_won"),
            Self::DisputeLost => write!(f, "dispute_lost"),
            Self::DisputeClosed => write!(f, "dispute_closed"),
            Self::ResolutionLost => write!(f, "resolution_lost"),
            Self::Drafted => write!(f, "drafted"),
            Self::Incomplete => write!(f, "incomplete"),
            Self::Unresolved => write!(f, "unresolved"),
            Self::OpenDispute => write!(f, "open_dispute"),
            Self::OpenResolution => write!(f, "open_resolution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
