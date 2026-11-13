pub use crate::prelude::*;

/// Why the trade failed, or `null`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TradeFailureCode {
    MarketUnavailable,
    LeverageTooHigh,
    AmountTooSmall,
    IsolatedPositionOpen,
    TradingPaused,
    FundingFailed,
    MarginUnavailable,
    BuilderFeeUnapproved,
    LeverageRejected,
    LeverageUnconfirmed,
    OrderRejected,
    CloseRejected,
    ReturnFailed,
    WithdrawFailed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TradeFailureCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MarketUnavailable => serializer.serialize_str("market_unavailable"),
            Self::LeverageTooHigh => serializer.serialize_str("leverage_too_high"),
            Self::AmountTooSmall => serializer.serialize_str("amount_too_small"),
            Self::IsolatedPositionOpen => serializer.serialize_str("isolated_position_open"),
            Self::TradingPaused => serializer.serialize_str("trading_paused"),
            Self::FundingFailed => serializer.serialize_str("funding_failed"),
            Self::MarginUnavailable => serializer.serialize_str("margin_unavailable"),
            Self::BuilderFeeUnapproved => serializer.serialize_str("builder_fee_unapproved"),
            Self::LeverageRejected => serializer.serialize_str("leverage_rejected"),
            Self::LeverageUnconfirmed => serializer.serialize_str("leverage_unconfirmed"),
            Self::OrderRejected => serializer.serialize_str("order_rejected"),
            Self::CloseRejected => serializer.serialize_str("close_rejected"),
            Self::ReturnFailed => serializer.serialize_str("return_failed"),
            Self::WithdrawFailed => serializer.serialize_str("withdraw_failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TradeFailureCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "market_unavailable" => Ok(Self::MarketUnavailable),
            "leverage_too_high" => Ok(Self::LeverageTooHigh),
            "amount_too_small" => Ok(Self::AmountTooSmall),
            "isolated_position_open" => Ok(Self::IsolatedPositionOpen),
            "trading_paused" => Ok(Self::TradingPaused),
            "funding_failed" => Ok(Self::FundingFailed),
            "margin_unavailable" => Ok(Self::MarginUnavailable),
            "builder_fee_unapproved" => Ok(Self::BuilderFeeUnapproved),
            "leverage_rejected" => Ok(Self::LeverageRejected),
            "leverage_unconfirmed" => Ok(Self::LeverageUnconfirmed),
            "order_rejected" => Ok(Self::OrderRejected),
            "close_rejected" => Ok(Self::CloseRejected),
            "return_failed" => Ok(Self::ReturnFailed),
            "withdraw_failed" => Ok(Self::WithdrawFailed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TradeFailureCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MarketUnavailable => write!(f, "market_unavailable"),
            Self::LeverageTooHigh => write!(f, "leverage_too_high"),
            Self::AmountTooSmall => write!(f, "amount_too_small"),
            Self::IsolatedPositionOpen => write!(f, "isolated_position_open"),
            Self::TradingPaused => write!(f, "trading_paused"),
            Self::FundingFailed => write!(f, "funding_failed"),
            Self::MarginUnavailable => write!(f, "margin_unavailable"),
            Self::BuilderFeeUnapproved => write!(f, "builder_fee_unapproved"),
            Self::LeverageRejected => write!(f, "leverage_rejected"),
            Self::LeverageUnconfirmed => write!(f, "leverage_unconfirmed"),
            Self::OrderRejected => write!(f, "order_rejected"),
            Self::CloseRejected => write!(f, "close_rejected"),
            Self::ReturnFailed => write!(f, "return_failed"),
            Self::WithdrawFailed => write!(f, "withdraw_failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
