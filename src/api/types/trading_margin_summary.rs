pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TradingMarginSummary {
    /// Total account value in USD, including unrealized profit and loss.
    #[serde(default)]
    pub account_value: Money,
    /// Margin allocated across open positions, in USD.
    #[serde(default)]
    pub total_margin_used: Money,
    /// Combined notional value of open positions, in USD.
    #[serde(default)]
    pub total_position_notional: Money,
    /// USD that can be withdrawn now without closing positions.
    #[serde(default)]
    pub withdrawable: Money,
}

impl TradingMarginSummary {
    pub fn builder() -> TradingMarginSummaryBuilder {
        <TradingMarginSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingMarginSummaryBuilder {
    account_value: Option<Money>,
    total_margin_used: Option<Money>,
    total_position_notional: Option<Money>,
    withdrawable: Option<Money>,
}

impl TradingMarginSummaryBuilder {
    pub fn account_value(mut self, value: Money) -> Self {
        self.account_value = Some(value);
        self
    }

    pub fn total_margin_used(mut self, value: Money) -> Self {
        self.total_margin_used = Some(value);
        self
    }

    pub fn total_position_notional(mut self, value: Money) -> Self {
        self.total_position_notional = Some(value);
        self
    }

    pub fn withdrawable(mut self, value: Money) -> Self {
        self.withdrawable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingMarginSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_value`](TradingMarginSummaryBuilder::account_value)
    /// - [`total_margin_used`](TradingMarginSummaryBuilder::total_margin_used)
    /// - [`total_position_notional`](TradingMarginSummaryBuilder::total_position_notional)
    /// - [`withdrawable`](TradingMarginSummaryBuilder::withdrawable)
    pub fn build(self) -> Result<TradingMarginSummary, BuildError> {
        Ok(TradingMarginSummary {
            account_value: self
                .account_value
                .ok_or_else(|| BuildError::missing_field("account_value"))?,
            total_margin_used: self
                .total_margin_used
                .ok_or_else(|| BuildError::missing_field("total_margin_used"))?,
            total_position_notional: self
                .total_position_notional
                .ok_or_else(|| BuildError::missing_field("total_position_notional"))?,
            withdrawable: self
                .withdrawable
                .ok_or_else(|| BuildError::missing_field("withdrawable"))?,
        })
    }
}
