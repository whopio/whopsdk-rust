pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountLevel {
    /// This account's own level from its points alone, from `0` up to `10`. `applied_level` is higher when the owner's other accounts lift it.
    #[serde(default)]
    pub account_level: i64,
    /// The level that applies: the highest the owner reached this month or in the 5 months before, from `0` up to `10`.
    #[serde(default)]
    pub applied_level: i64,
    /// `true` when this account's own points this month fall short of `applied_level`, so the owner's other accounts lift it to that level.
    #[serde(default)]
    pub boosted: bool,
    #[serde(default)]
    pub levels: Vec<AccountProcessingOfferLevel>,
    /// Whop Ads charges paid this calendar month across every account the owner owns, net of refunds, in USD, so it can be negative. An account whose month nets to zero or below is left out.
    #[serde(default)]
    pub monthly_ads_spend: Money,
    /// Captured Whop Card spend this calendar month across every account the owner owns, net of refunds, in USD, so it can be negative. An account whose month nets to zero or below is left out.
    #[serde(default)]
    pub monthly_card_spend: Money,
    /// Qualifying spend this calendar month across every account the owner owns, plus the owner's personal trading and partner points, in USD: `points` divided by 10.
    #[serde(default)]
    pub monthly_qualifying_spend: Money,
    /// Perpetual trading volume filled through Whop this calendar month across every account the owner owns and their personal wallet, in USD. An account whose month nets to zero or below is left out.
    #[serde(default)]
    pub monthly_trading_volume: Money,
    /// Points earned this month across every account the owner owns, plus the owner's partner points, rounded down. An account whose month nets below zero counts as zero.
    #[serde(default)]
    pub points: i64,
    /// This month's points, split by where they were earned. Each source is net of its refunds, so one can be negative.
    #[serde(default)]
    pub points_by_source: OwnerLevelPoints,
    /// Points still needed this month to reach the next level, or `null` at level `10`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points_to_next: Option<i64>,
    /// When this month's points reset to zero, as an ISO 8601 timestamp.
    #[serde(default)]
    pub resets_at: String,
}

impl AccountLevel {
    pub fn builder() -> AccountLevelBuilder {
        <AccountLevelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountLevelBuilder {
    account_level: Option<i64>,
    applied_level: Option<i64>,
    boosted: Option<bool>,
    levels: Option<Vec<AccountProcessingOfferLevel>>,
    monthly_ads_spend: Option<Money>,
    monthly_card_spend: Option<Money>,
    monthly_qualifying_spend: Option<Money>,
    monthly_trading_volume: Option<Money>,
    points: Option<i64>,
    points_by_source: Option<OwnerLevelPoints>,
    points_to_next: Option<i64>,
    resets_at: Option<String>,
}

impl AccountLevelBuilder {
    pub fn account_level(mut self, value: i64) -> Self {
        self.account_level = Some(value);
        self
    }

    pub fn applied_level(mut self, value: i64) -> Self {
        self.applied_level = Some(value);
        self
    }

    pub fn boosted(mut self, value: bool) -> Self {
        self.boosted = Some(value);
        self
    }

    pub fn levels(mut self, value: Vec<AccountProcessingOfferLevel>) -> Self {
        self.levels = Some(value);
        self
    }

    pub fn monthly_ads_spend(mut self, value: Money) -> Self {
        self.monthly_ads_spend = Some(value);
        self
    }

    pub fn monthly_card_spend(mut self, value: Money) -> Self {
        self.monthly_card_spend = Some(value);
        self
    }

    pub fn monthly_qualifying_spend(mut self, value: Money) -> Self {
        self.monthly_qualifying_spend = Some(value);
        self
    }

    pub fn monthly_trading_volume(mut self, value: Money) -> Self {
        self.monthly_trading_volume = Some(value);
        self
    }

    pub fn points(mut self, value: i64) -> Self {
        self.points = Some(value);
        self
    }

    pub fn points_by_source(mut self, value: OwnerLevelPoints) -> Self {
        self.points_by_source = Some(value);
        self
    }

    pub fn points_to_next(mut self, value: i64) -> Self {
        self.points_to_next = Some(value);
        self
    }

    pub fn resets_at(mut self, value: impl Into<String>) -> Self {
        self.resets_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountLevel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_level`](AccountLevelBuilder::account_level)
    /// - [`applied_level`](AccountLevelBuilder::applied_level)
    /// - [`boosted`](AccountLevelBuilder::boosted)
    /// - [`levels`](AccountLevelBuilder::levels)
    /// - [`monthly_ads_spend`](AccountLevelBuilder::monthly_ads_spend)
    /// - [`monthly_card_spend`](AccountLevelBuilder::monthly_card_spend)
    /// - [`monthly_qualifying_spend`](AccountLevelBuilder::monthly_qualifying_spend)
    /// - [`monthly_trading_volume`](AccountLevelBuilder::monthly_trading_volume)
    /// - [`points`](AccountLevelBuilder::points)
    /// - [`points_by_source`](AccountLevelBuilder::points_by_source)
    /// - [`resets_at`](AccountLevelBuilder::resets_at)
    pub fn build(self) -> Result<AccountLevel, BuildError> {
        Ok(AccountLevel {
            account_level: self
                .account_level
                .ok_or_else(|| BuildError::missing_field("account_level"))?,
            applied_level: self
                .applied_level
                .ok_or_else(|| BuildError::missing_field("applied_level"))?,
            boosted: self
                .boosted
                .ok_or_else(|| BuildError::missing_field("boosted"))?,
            levels: self
                .levels
                .ok_or_else(|| BuildError::missing_field("levels"))?,
            monthly_ads_spend: self
                .monthly_ads_spend
                .ok_or_else(|| BuildError::missing_field("monthly_ads_spend"))?,
            monthly_card_spend: self
                .monthly_card_spend
                .ok_or_else(|| BuildError::missing_field("monthly_card_spend"))?,
            monthly_qualifying_spend: self
                .monthly_qualifying_spend
                .ok_or_else(|| BuildError::missing_field("monthly_qualifying_spend"))?,
            monthly_trading_volume: self
                .monthly_trading_volume
                .ok_or_else(|| BuildError::missing_field("monthly_trading_volume"))?,
            points: self
                .points
                .ok_or_else(|| BuildError::missing_field("points"))?,
            points_by_source: self
                .points_by_source
                .ok_or_else(|| BuildError::missing_field("points_by_source"))?,
            points_to_next: self.points_to_next,
            resets_at: self
                .resets_at
                .ok_or_else(|| BuildError::missing_field("resets_at"))?,
        })
    }
}
