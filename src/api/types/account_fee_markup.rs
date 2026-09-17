pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountFeeMarkup {
    /// Whether the caller may change this markup through `PATCH`. True for the platform's team holding the `company:update_child_fees` scope.
    #[serde(default)]
    pub adjustable: bool,
    /// What applies if this row is cleared: the platform's default for all its connected accounts, or zero.
    #[serde(default)]
    pub default: AccountFeeRate,
    /// The amount the platform adds per event. Zero when no markup is set.
    #[serde(default)]
    pub fixed: Money,
    /// The highest markup the platform may set.
    #[serde(default)]
    pub maximum: AccountFeeRate,
    /// The percentage of the transaction the platform adds, where `2` means 2%. `0` when no markup is set.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percentage: f64,
    /// `custom` when a row is set at this level, `default` when the rate falls through to the platform default or zero.
    pub source: AccountFeeMarkupSource,
    /// Why the caller may not change this markup, or `null` when `adjustable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unadjustable_reason: Option<AccountFeeMarkupUnadjustableReason>,
}

impl AccountFeeMarkup {
    pub fn builder() -> AccountFeeMarkupBuilder {
        <AccountFeeMarkupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeeMarkupBuilder {
    adjustable: Option<bool>,
    default: Option<AccountFeeRate>,
    fixed: Option<Money>,
    maximum: Option<AccountFeeRate>,
    percentage: Option<f64>,
    source: Option<AccountFeeMarkupSource>,
    unadjustable_reason: Option<AccountFeeMarkupUnadjustableReason>,
}

impl AccountFeeMarkupBuilder {
    pub fn adjustable(mut self, value: bool) -> Self {
        self.adjustable = Some(value);
        self
    }

    pub fn default(mut self, value: AccountFeeRate) -> Self {
        self.default = Some(value);
        self
    }

    pub fn fixed(mut self, value: Money) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn maximum(mut self, value: AccountFeeRate) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn source(mut self, value: AccountFeeMarkupSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn unadjustable_reason(mut self, value: AccountFeeMarkupUnadjustableReason) -> Self {
        self.unadjustable_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFeeMarkup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adjustable`](AccountFeeMarkupBuilder::adjustable)
    /// - [`default`](AccountFeeMarkupBuilder::default)
    /// - [`fixed`](AccountFeeMarkupBuilder::fixed)
    /// - [`maximum`](AccountFeeMarkupBuilder::maximum)
    /// - [`percentage`](AccountFeeMarkupBuilder::percentage)
    /// - [`source`](AccountFeeMarkupBuilder::source)
    pub fn build(self) -> Result<AccountFeeMarkup, BuildError> {
        Ok(AccountFeeMarkup {
            adjustable: self
                .adjustable
                .ok_or_else(|| BuildError::missing_field("adjustable"))?,
            default: self
                .default
                .ok_or_else(|| BuildError::missing_field("default"))?,
            fixed: self
                .fixed
                .ok_or_else(|| BuildError::missing_field("fixed"))?,
            maximum: self
                .maximum
                .ok_or_else(|| BuildError::missing_field("maximum"))?,
            percentage: self
                .percentage
                .ok_or_else(|| BuildError::missing_field("percentage"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            unadjustable_reason: self.unadjustable_reason,
        })
    }
}
