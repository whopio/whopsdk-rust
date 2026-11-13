pub use crate::prelude::*;

/// Where the registration is: awaiting payment, ordering at the registrar, transferring from another registrar, delegating DNS to Whop, held, expired, or releasing.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainRegistrationPhase {
    AwaitingPayment,
    Ordering,
    Transferring,
    Delegating,
    Held,
    Expired,
    Failed,
    Releasing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainRegistrationPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AwaitingPayment => serializer.serialize_str("awaiting_payment"),
            Self::Ordering => serializer.serialize_str("ordering"),
            Self::Transferring => serializer.serialize_str("transferring"),
            Self::Delegating => serializer.serialize_str("delegating"),
            Self::Held => serializer.serialize_str("held"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Releasing => serializer.serialize_str("releasing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainRegistrationPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "awaiting_payment" => Ok(Self::AwaitingPayment),
            "ordering" => Ok(Self::Ordering),
            "transferring" => Ok(Self::Transferring),
            "delegating" => Ok(Self::Delegating),
            "held" => Ok(Self::Held),
            "expired" => Ok(Self::Expired),
            "failed" => Ok(Self::Failed),
            "releasing" => Ok(Self::Releasing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainRegistrationPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AwaitingPayment => write!(f, "awaiting_payment"),
            Self::Ordering => write!(f, "ordering"),
            Self::Transferring => write!(f, "transferring"),
            Self::Delegating => write!(f, "delegating"),
            Self::Held => write!(f, "held"),
            Self::Expired => write!(f, "expired"),
            Self::Failed => write!(f, "failed"),
            Self::Releasing => write!(f, "releasing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
