pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponseNearbyItem {
    /// When the referrer's earliest partner business became active.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub first_referral_started_at: DateTime<FixedOffset>,
    /// 1-based leaderboard position.
    #[serde(default)]
    pub rank: i64,
    /// The referrer's pending + completed earnings across all referred businesses, in USD, without commission advances. Whop Sales commission counts only the share that reaches its people.
    #[serde(default)]
    pub total_earnings_usd: String,
    /// Credited GMV across all the referrer's referred businesses, in USD.
    #[serde(default)]
    pub total_volume_usd: String,
    /// The ranked referrer. Identity fields (id, name, username, profile_picture) are returned only on the caller's own entry; other referrers expose coarse location only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<LeaderboardPartnersResponseNearbyItemUser>,
}

impl LeaderboardPartnersResponseNearbyItem {
    pub fn builder() -> LeaderboardPartnersResponseNearbyItemBuilder {
        <LeaderboardPartnersResponseNearbyItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseNearbyItemBuilder {
    first_referral_started_at: Option<DateTime<FixedOffset>>,
    rank: Option<i64>,
    total_earnings_usd: Option<String>,
    total_volume_usd: Option<String>,
    user: Option<LeaderboardPartnersResponseNearbyItemUser>,
}

impl LeaderboardPartnersResponseNearbyItemBuilder {
    pub fn first_referral_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_referral_started_at = Some(value);
        self
    }

    pub fn rank(mut self, value: i64) -> Self {
        self.rank = Some(value);
        self
    }

    pub fn total_earnings_usd(mut self, value: impl Into<String>) -> Self {
        self.total_earnings_usd = Some(value.into());
        self
    }

    pub fn total_volume_usd(mut self, value: impl Into<String>) -> Self {
        self.total_volume_usd = Some(value.into());
        self
    }

    pub fn user(mut self, value: LeaderboardPartnersResponseNearbyItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponseNearbyItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_referral_started_at`](LeaderboardPartnersResponseNearbyItemBuilder::first_referral_started_at)
    /// - [`rank`](LeaderboardPartnersResponseNearbyItemBuilder::rank)
    /// - [`total_earnings_usd`](LeaderboardPartnersResponseNearbyItemBuilder::total_earnings_usd)
    /// - [`total_volume_usd`](LeaderboardPartnersResponseNearbyItemBuilder::total_volume_usd)
    pub fn build(self) -> Result<LeaderboardPartnersResponseNearbyItem, BuildError> {
        Ok(LeaderboardPartnersResponseNearbyItem {
            first_referral_started_at: self
                .first_referral_started_at
                .ok_or_else(|| BuildError::missing_field("first_referral_started_at"))?,
            rank: self.rank.ok_or_else(|| BuildError::missing_field("rank"))?,
            total_earnings_usd: self
                .total_earnings_usd
                .ok_or_else(|| BuildError::missing_field("total_earnings_usd"))?,
            total_volume_usd: self
                .total_volume_usd
                .ok_or_else(|| BuildError::missing_field("total_volume_usd"))?,
            user: self.user,
        })
    }
}
