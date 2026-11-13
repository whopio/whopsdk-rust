pub use crate::prelude::*;

/// The membership's latest lifecycle event, such as `renewing_on` or `canceled_on`. Pair it with `most_recent_action_at`. `null` when none is recorded.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipMostRecentAction {
    CanceledOn,
    FinishedSplitPayAt,
    CancelingOn,
    EndingOn,
    RenewingOn,
    PastDue,
    NextPaymentAttempt,
    PausedAt,
    TrialEndsAt,
    UnresolvedAt,
    TrialCancelingOn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipMostRecentAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CanceledOn => serializer.serialize_str("canceled_on"),
            Self::FinishedSplitPayAt => serializer.serialize_str("finished_split_pay_at"),
            Self::CancelingOn => serializer.serialize_str("canceling_on"),
            Self::EndingOn => serializer.serialize_str("ending_on"),
            Self::RenewingOn => serializer.serialize_str("renewing_on"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::NextPaymentAttempt => serializer.serialize_str("next_payment_attempt"),
            Self::PausedAt => serializer.serialize_str("paused_at"),
            Self::TrialEndsAt => serializer.serialize_str("trial_ends_at"),
            Self::UnresolvedAt => serializer.serialize_str("unresolved_at"),
            Self::TrialCancelingOn => serializer.serialize_str("trial_canceling_on"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipMostRecentAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "canceled_on" => Ok(Self::CanceledOn),
            "finished_split_pay_at" => Ok(Self::FinishedSplitPayAt),
            "canceling_on" => Ok(Self::CancelingOn),
            "ending_on" => Ok(Self::EndingOn),
            "renewing_on" => Ok(Self::RenewingOn),
            "past_due" => Ok(Self::PastDue),
            "next_payment_attempt" => Ok(Self::NextPaymentAttempt),
            "paused_at" => Ok(Self::PausedAt),
            "trial_ends_at" => Ok(Self::TrialEndsAt),
            "unresolved_at" => Ok(Self::UnresolvedAt),
            "trial_canceling_on" => Ok(Self::TrialCancelingOn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipMostRecentAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanceledOn => write!(f, "canceled_on"),
            Self::FinishedSplitPayAt => write!(f, "finished_split_pay_at"),
            Self::CancelingOn => write!(f, "canceling_on"),
            Self::EndingOn => write!(f, "ending_on"),
            Self::RenewingOn => write!(f, "renewing_on"),
            Self::PastDue => write!(f, "past_due"),
            Self::NextPaymentAttempt => write!(f, "next_payment_attempt"),
            Self::PausedAt => write!(f, "paused_at"),
            Self::TrialEndsAt => write!(f, "trial_ends_at"),
            Self::UnresolvedAt => write!(f, "unresolved_at"),
            Self::TrialCancelingOn => write!(f, "trial_canceling_on"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
