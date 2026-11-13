pub use crate::prelude::*;

/// Settle every new sale into this currency, regardless of the plan's currency or what the buyer paid in; sales converted this way carry an additional 0.5% FX fee. Pass `null` to go back to settling each sale in its plan's currency. Changing it never converts money already in your balances, and it can change at most once a day (see `preferred_settlement_currency_changeable_at`). Requires the `payout:account:update` scope on your API key.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesRequestPreferredSettlementCurrency {
    Usd,
    Eur,
    Gbp,
    Cad,
    Aud,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdatePreferencesRequestPreferredSettlementCurrency {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Usd => serializer.serialize_str("usd"),
            Self::Eur => serializer.serialize_str("eur"),
            Self::Gbp => serializer.serialize_str("gbp"),
            Self::Cad => serializer.serialize_str("cad"),
            Self::Aud => serializer.serialize_str("aud"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdatePreferencesRequestPreferredSettlementCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "usd" => Ok(Self::Usd),
            "eur" => Ok(Self::Eur),
            "gbp" => Ok(Self::Gbp),
            "cad" => Ok(Self::Cad),
            "aud" => Ok(Self::Aud),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdatePreferencesRequestPreferredSettlementCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usd => write!(f, "usd"),
            Self::Eur => write!(f, "eur"),
            Self::Gbp => write!(f, "gbp"),
            Self::Cad => write!(f, "cad"),
            Self::Aud => write!(f, "aud"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
