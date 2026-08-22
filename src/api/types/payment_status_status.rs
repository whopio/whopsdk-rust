pub use crate::prelude::*;

/// How far the payment has got. `requires_confirmation` — nothing attempted yet, or the last attempt failed and can be retried. `requires_action` — the buyer has a step outstanding; see `next_action`. `confirming` — the buyer has done their part and the processor is deciding. `processing` — the money is moving; see `processing_details`. `succeeded` — collected. `canceled` — voided or written off.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentStatusStatus {
    RequiresConfirmation,
    RequiresAction,
    Confirming,
    Processing,
    Succeeded,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentStatusStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequiresConfirmation => serializer.serialize_str("requires_confirmation"),
            Self::RequiresAction => serializer.serialize_str("requires_action"),
            Self::Confirming => serializer.serialize_str("confirming"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requires_confirmation" => Ok(Self::RequiresConfirmation),
            "requires_action" => Ok(Self::RequiresAction),
            "confirming" => Ok(Self::Confirming),
            "processing" => Ok(Self::Processing),
            "succeeded" => Ok(Self::Succeeded),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiresConfirmation => write!(f, "requires_confirmation"),
            Self::RequiresAction => write!(f, "requires_action"),
            Self::Confirming => write!(f, "confirming"),
            Self::Processing => write!(f, "processing"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
