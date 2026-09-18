pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountFeeRegionalRate {
    /// The platform rate for this region before custom or inherited pricing is applied.
    #[serde(default)]
    pub default: AccountFeeRate,
    /// The amount charged per event in effect. `null` when the fee has no fixed component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Money>,
    /// The lowest regional rate the caller may set, present only when the fee is adjustable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<AccountFeeRate>,
    /// The percentage of the transaction in effect, where `2` means 2%. `null` when the fee has no percentage component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
    /// The regional rate that takes effect when this account's custom rate is cleared, including inherited pricing.
    #[serde(default)]
    pub reset: AccountFeeRate,
    /// Where the regional rate in effect comes from: `default` is the platform rate, `custom` a rate negotiated for this account, and `inherited` a rate negotiated by the platform this account is connected to.
    pub source: AccountFeeRegionalRateSource,
}

impl AccountFeeRegionalRate {
    pub fn builder() -> AccountFeeRegionalRateBuilder {
        <AccountFeeRegionalRateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeeRegionalRateBuilder {
    default: Option<AccountFeeRate>,
    fixed: Option<Money>,
    minimum: Option<AccountFeeRate>,
    percentage: Option<f64>,
    reset: Option<AccountFeeRate>,
    source: Option<AccountFeeRegionalRateSource>,
}

impl AccountFeeRegionalRateBuilder {
    pub fn default(mut self, value: AccountFeeRate) -> Self {
        self.default = Some(value);
        self
    }

    pub fn fixed(mut self, value: Money) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn minimum(mut self, value: AccountFeeRate) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn reset(mut self, value: AccountFeeRate) -> Self {
        self.reset = Some(value);
        self
    }

    pub fn source(mut self, value: AccountFeeRegionalRateSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFeeRegionalRate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default`](AccountFeeRegionalRateBuilder::default)
    /// - [`reset`](AccountFeeRegionalRateBuilder::reset)
    /// - [`source`](AccountFeeRegionalRateBuilder::source)
    pub fn build(self) -> Result<AccountFeeRegionalRate, BuildError> {
        Ok(AccountFeeRegionalRate {
            default: self
                .default
                .ok_or_else(|| BuildError::missing_field("default"))?,
            fixed: self.fixed,
            minimum: self.minimum,
            percentage: self.percentage,
            reset: self
                .reset
                .ok_or_else(|| BuildError::missing_field("reset"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
