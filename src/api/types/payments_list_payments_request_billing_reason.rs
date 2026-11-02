pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPaymentsRequestBillingReason {
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionUpdate,
    OneTime,
    Manual,
    Subscription,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPaymentsRequestBillingReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SubscriptionCreate => serializer.serialize_str("subscription_create"),
            Self::SubscriptionCycle => serializer.serialize_str("subscription_cycle"),
            Self::SubscriptionUpdate => serializer.serialize_str("subscription_update"),
            Self::OneTime => serializer.serialize_str("one_time"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::Subscription => serializer.serialize_str("subscription"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPaymentsRequestBillingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subscription_create" => Ok(Self::SubscriptionCreate),
            "subscription_cycle" => Ok(Self::SubscriptionCycle),
            "subscription_update" => Ok(Self::SubscriptionUpdate),
            "one_time" => Ok(Self::OneTime),
            "manual" => Ok(Self::Manual),
            "subscription" => Ok(Self::Subscription),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPaymentsRequestBillingReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SubscriptionCreate => write!(f, "subscription_create"),
            Self::SubscriptionCycle => write!(f, "subscription_cycle"),
            Self::SubscriptionUpdate => write!(f, "subscription_update"),
            Self::OneTime => write!(f, "one_time"),
            Self::Manual => write!(f, "manual"),
            Self::Subscription => write!(f, "subscription"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
