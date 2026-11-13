pub use crate::prelude::*;

/// Why the payment can't earn a rebate: `not_card_payment`, `before_levels` (made before Whop Levels started), `not_in_levels` (its account wasn't in Whop Levels that month), `fee_not_posted`, `fee_at_level_rate` (its fee was already at or below the level's rate) or `no_level_rate` (its fee matched no level rate). `null` unless `status` is `ineligible`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentLevelsRebateIneligibleReason {
    NotCardPayment,
    BeforeLevels,
    NotInLevels,
    FeeNotPosted,
    FeeAtLevelRate,
    NoLevelRate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentLevelsRebateIneligibleReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NotCardPayment => serializer.serialize_str("not_card_payment"),
            Self::BeforeLevels => serializer.serialize_str("before_levels"),
            Self::NotInLevels => serializer.serialize_str("not_in_levels"),
            Self::FeeNotPosted => serializer.serialize_str("fee_not_posted"),
            Self::FeeAtLevelRate => serializer.serialize_str("fee_at_level_rate"),
            Self::NoLevelRate => serializer.serialize_str("no_level_rate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentLevelsRebateIneligibleReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "not_card_payment" => Ok(Self::NotCardPayment),
            "before_levels" => Ok(Self::BeforeLevels),
            "not_in_levels" => Ok(Self::NotInLevels),
            "fee_not_posted" => Ok(Self::FeeNotPosted),
            "fee_at_level_rate" => Ok(Self::FeeAtLevelRate),
            "no_level_rate" => Ok(Self::NoLevelRate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentLevelsRebateIneligibleReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotCardPayment => write!(f, "not_card_payment"),
            Self::BeforeLevels => write!(f, "before_levels"),
            Self::NotInLevels => write!(f, "not_in_levels"),
            Self::FeeNotPosted => write!(f, "fee_not_posted"),
            Self::FeeAtLevelRate => write!(f, "fee_at_level_rate"),
            Self::NoLevelRate => write!(f, "no_level_rate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
