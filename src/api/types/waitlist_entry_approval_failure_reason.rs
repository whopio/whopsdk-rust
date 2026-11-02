pub use crate::prelude::*;

/// Why the last approval attempt failed, or `null` when none has. `plan_unavailable` — the plan, product, or seller account was deleted. `already_member` — the user already has a membership on a one-per-user product. `checkout_failed` — checkout failed, usually a declined payment, and the signup was denied. `unknown` — another failure; retry. Cleared when approval is requeued.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WaitlistEntryApprovalFailureReason {
    PlanUnavailable,
    AlreadyMember,
    CheckoutFailed,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WaitlistEntryApprovalFailureReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PlanUnavailable => serializer.serialize_str("plan_unavailable"),
            Self::AlreadyMember => serializer.serialize_str("already_member"),
            Self::CheckoutFailed => serializer.serialize_str("checkout_failed"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WaitlistEntryApprovalFailureReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "plan_unavailable" => Ok(Self::PlanUnavailable),
            "already_member" => Ok(Self::AlreadyMember),
            "checkout_failed" => Ok(Self::CheckoutFailed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WaitlistEntryApprovalFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlanUnavailable => write!(f, "plan_unavailable"),
            Self::AlreadyMember => write!(f, "already_member"),
            Self::CheckoutFailed => write!(f, "checkout_failed"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
