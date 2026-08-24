pub use crate::prelude::*;

/// The different most recent actions a member can have.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MemberMostRecentActions {
    Canceling,
    Churned,
    FinishedSplitPay,
    Paused,
    PaidSubscriber,
    PaidOnce,
    Expiring,
    Joined,
    Drafted,
    Left,
    Trialing,
    PendingEntry,
    Renewing,
    PastDue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MemberMostRecentActions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::Churned => serializer.serialize_str("churned"),
            Self::FinishedSplitPay => serializer.serialize_str("finished_split_pay"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::PaidSubscriber => serializer.serialize_str("paid_subscriber"),
            Self::PaidOnce => serializer.serialize_str("paid_once"),
            Self::Expiring => serializer.serialize_str("expiring"),
            Self::Joined => serializer.serialize_str("joined"),
            Self::Drafted => serializer.serialize_str("drafted"),
            Self::Left => serializer.serialize_str("left"),
            Self::Trialing => serializer.serialize_str("trialing"),
            Self::PendingEntry => serializer.serialize_str("pending_entry"),
            Self::Renewing => serializer.serialize_str("renewing"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MemberMostRecentActions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "canceling" => Ok(Self::Canceling),
            "churned" => Ok(Self::Churned),
            "finished_split_pay" => Ok(Self::FinishedSplitPay),
            "paused" => Ok(Self::Paused),
            "paid_subscriber" => Ok(Self::PaidSubscriber),
            "paid_once" => Ok(Self::PaidOnce),
            "expiring" => Ok(Self::Expiring),
            "joined" => Ok(Self::Joined),
            "drafted" => Ok(Self::Drafted),
            "left" => Ok(Self::Left),
            "trialing" => Ok(Self::Trialing),
            "pending_entry" => Ok(Self::PendingEntry),
            "renewing" => Ok(Self::Renewing),
            "past_due" => Ok(Self::PastDue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MemberMostRecentActions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canceling => write!(f, "canceling"),
            Self::Churned => write!(f, "churned"),
            Self::FinishedSplitPay => write!(f, "finished_split_pay"),
            Self::Paused => write!(f, "paused"),
            Self::PaidSubscriber => write!(f, "paid_subscriber"),
            Self::PaidOnce => write!(f, "paid_once"),
            Self::Expiring => write!(f, "expiring"),
            Self::Joined => write!(f, "joined"),
            Self::Drafted => write!(f, "drafted"),
            Self::Left => write!(f, "left"),
            Self::Trialing => write!(f, "trialing"),
            Self::PendingEntry => write!(f, "pending_entry"),
            Self::Renewing => write!(f, "renewing"),
            Self::PastDue => write!(f, "past_due"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
