pub use crate::prelude::*;

/// The different statuses a payment transaction can be in.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentTransactionStatuses {
    Succeeded,
    Declined,
    Error,
    Pending,
    Created,
    Expired,
    Won,
    Rejected,
    Lost,
    Prevented,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentTransactionStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::Error => serializer.serialize_str("error"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Created => serializer.serialize_str("created"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Won => serializer.serialize_str("won"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Lost => serializer.serialize_str("lost"),
            Self::Prevented => serializer.serialize_str("prevented"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentTransactionStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "succeeded" => Ok(Self::Succeeded),
            "declined" => Ok(Self::Declined),
            "error" => Ok(Self::Error),
            "pending" => Ok(Self::Pending),
            "created" => Ok(Self::Created),
            "expired" => Ok(Self::Expired),
            "won" => Ok(Self::Won),
            "rejected" => Ok(Self::Rejected),
            "lost" => Ok(Self::Lost),
            "prevented" => Ok(Self::Prevented),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentTransactionStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Succeeded => write!(f, "succeeded"),
            Self::Declined => write!(f, "declined"),
            Self::Error => write!(f, "error"),
            Self::Pending => write!(f, "pending"),
            Self::Created => write!(f, "created"),
            Self::Expired => write!(f, "expired"),
            Self::Won => write!(f, "won"),
            Self::Rejected => write!(f, "rejected"),
            Self::Lost => write!(f, "lost"),
            Self::Prevented => write!(f, "prevented"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
