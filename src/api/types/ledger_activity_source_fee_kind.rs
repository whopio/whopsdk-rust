pub use crate::prelude::*;

/// Action that generated a platform markup fee: deposit, swap, transfer, card_spend, or payout. Present for platform_markup_fee and platform_markup_fee_payout, including when include_resource is false. Null when the originating action is unavailable; omitted on other source types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LedgerActivitySourceFeeKind {
    Payout,
    Transfer,
    Deposit,
    Swap,
    CardSpend,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LedgerActivitySourceFeeKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Payout => serializer.serialize_str("payout"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::Deposit => serializer.serialize_str("deposit"),
            Self::Swap => serializer.serialize_str("swap"),
            Self::CardSpend => serializer.serialize_str("card_spend"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LedgerActivitySourceFeeKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payout" => Ok(Self::Payout),
            "transfer" => Ok(Self::Transfer),
            "deposit" => Ok(Self::Deposit),
            "swap" => Ok(Self::Swap),
            "card_spend" => Ok(Self::CardSpend),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LedgerActivitySourceFeeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Payout => write!(f, "payout"),
            Self::Transfer => write!(f, "transfer"),
            Self::Deposit => write!(f, "deposit"),
            Self::Swap => write!(f, "swap"),
            Self::CardSpend => write!(f, "card_spend"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
