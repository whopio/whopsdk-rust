pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountFee {
    /// Whether the caller may change this fee through `PATCH`. Depends on who is asking.
    #[serde(default)]
    pub adjustable: bool,
    /// Which group of the fee schedule this fee belongs to, for grouping in a UI.
    pub category: AccountFeeCategory,
    /// The platform rate with no custom deal: what applies if the custom rate is cleared.
    #[serde(default)]
    pub default: AccountFeeRate,
    /// When a custom or inherited rate expires and the fee returns to `default`, as an ISO 8601 timestamp. `null` when the default applies or the rate does not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// The amount charged per event in effect. `null` when the fee has no fixed component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Money>,
    /// The lowest rate the caller may set, present only when `adjustable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<AccountFeeRate>,
    /// The percentage of the transaction in effect, where `2` means 2%. `null` when the fee has no percentage component.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
    /// The acquirer region `percentage` and `fixed` describe, for a fee that varies by where the money is processed. `null` for a fee that does not vary by region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<AccountFeeRegion>,
    /// The rate in every other region this fee varies by, keyed by region. Empty for a fee that does not vary by region.
    #[serde(default)]
    pub regions: HashMap<String, AccountFeeRate>,
    /// Where the rate in effect comes from: `default` is the platform rate, `custom` a rate negotiated for this account, and `inherited` a rate negotiated by the platform this account is connected to.
    pub source: AccountFeeSource,
    /// Why the caller may not change this fee, or `null` when `adjustable`. `not_permitted` when the caller has no say over it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unadjustable_reason: Option<AccountFeeUnadjustableReason>,
}

impl AccountFee {
    pub fn builder() -> AccountFeeBuilder {
        <AccountFeeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountFeeBuilder {
    adjustable: Option<bool>,
    category: Option<AccountFeeCategory>,
    default: Option<AccountFeeRate>,
    ends_at: Option<String>,
    fixed: Option<Money>,
    minimum: Option<AccountFeeRate>,
    percentage: Option<f64>,
    region: Option<AccountFeeRegion>,
    regions: Option<HashMap<String, AccountFeeRate>>,
    source: Option<AccountFeeSource>,
    unadjustable_reason: Option<AccountFeeUnadjustableReason>,
}

impl AccountFeeBuilder {
    pub fn adjustable(mut self, value: bool) -> Self {
        self.adjustable = Some(value);
        self
    }

    pub fn category(mut self, value: AccountFeeCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn default(mut self, value: AccountFeeRate) -> Self {
        self.default = Some(value);
        self
    }

    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
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

    pub fn region(mut self, value: AccountFeeRegion) -> Self {
        self.region = Some(value);
        self
    }

    pub fn regions(mut self, value: HashMap<String, AccountFeeRate>) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn source(mut self, value: AccountFeeSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn unadjustable_reason(mut self, value: AccountFeeUnadjustableReason) -> Self {
        self.unadjustable_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountFee`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adjustable`](AccountFeeBuilder::adjustable)
    /// - [`category`](AccountFeeBuilder::category)
    /// - [`default`](AccountFeeBuilder::default)
    /// - [`regions`](AccountFeeBuilder::regions)
    /// - [`source`](AccountFeeBuilder::source)
    pub fn build(self) -> Result<AccountFee, BuildError> {
        Ok(AccountFee {
            adjustable: self
                .adjustable
                .ok_or_else(|| BuildError::missing_field("adjustable"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            default: self
                .default
                .ok_or_else(|| BuildError::missing_field("default"))?,
            ends_at: self.ends_at,
            fixed: self.fixed,
            minimum: self.minimum,
            percentage: self.percentage,
            region: self.region,
            regions: self
                .regions
                .ok_or_else(|| BuildError::missing_field("regions"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            unadjustable_reason: self.unadjustable_reason,
        })
    }
}
