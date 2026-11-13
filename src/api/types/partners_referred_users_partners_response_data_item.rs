pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersPartnersResponseDataItem {
    /// Number of active businesses this user referred that credit the caller as a second-tier partner, or any earning partner in global mode. An earning-partner filter narrows the count to that partner. Excludes deleted businesses.
    #[serde(default)]
    pub business_count: i64,
    /// Total pending and completed affiliate earnings from this user for their current primary referrer, across all tiers, in USD. Includes historical earnings from removed referrals and deleted businesses.
    #[serde(default)]
    pub earnings: Money,
    /// When the referred user joined Whop, as an ISO 8601 timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub joined_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub total_earnings_usd: String,
    #[serde(default)]
    pub total_volume_usd: String,
    #[serde(default)]
    pub user: ReferredUsersPartnersResponseDataItemUser,
    /// Attributed volume over the last 30 days across the business referrals included in business_count, in USD.
    #[serde(rename = "volume_30d")]
    #[serde(default)]
    pub volume30d: Money,
}

impl ReferredUsersPartnersResponseDataItem {
    pub fn builder() -> ReferredUsersPartnersResponseDataItemBuilder {
        <ReferredUsersPartnersResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersPartnersResponseDataItemBuilder {
    business_count: Option<i64>,
    earnings: Option<Money>,
    joined_at: Option<DateTime<FixedOffset>>,
    total_earnings_usd: Option<String>,
    total_volume_usd: Option<String>,
    user: Option<ReferredUsersPartnersResponseDataItemUser>,
    volume30d: Option<Money>,
}

impl ReferredUsersPartnersResponseDataItemBuilder {
    pub fn business_count(mut self, value: i64) -> Self {
        self.business_count = Some(value);
        self
    }

    pub fn earnings(mut self, value: Money) -> Self {
        self.earnings = Some(value);
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
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

    pub fn user(mut self, value: ReferredUsersPartnersResponseDataItemUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn volume30d(mut self, value: Money) -> Self {
        self.volume30d = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReferredUsersPartnersResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_count`](ReferredUsersPartnersResponseDataItemBuilder::business_count)
    /// - [`earnings`](ReferredUsersPartnersResponseDataItemBuilder::earnings)
    /// - [`joined_at`](ReferredUsersPartnersResponseDataItemBuilder::joined_at)
    /// - [`total_earnings_usd`](ReferredUsersPartnersResponseDataItemBuilder::total_earnings_usd)
    /// - [`total_volume_usd`](ReferredUsersPartnersResponseDataItemBuilder::total_volume_usd)
    /// - [`user`](ReferredUsersPartnersResponseDataItemBuilder::user)
    /// - [`volume30d`](ReferredUsersPartnersResponseDataItemBuilder::volume30d)
    pub fn build(self) -> Result<ReferredUsersPartnersResponseDataItem, BuildError> {
        Ok(ReferredUsersPartnersResponseDataItem {
            business_count: self
                .business_count
                .ok_or_else(|| BuildError::missing_field("business_count"))?,
            earnings: self
                .earnings
                .ok_or_else(|| BuildError::missing_field("earnings"))?,
            joined_at: self
                .joined_at
                .ok_or_else(|| BuildError::missing_field("joined_at"))?,
            total_earnings_usd: self
                .total_earnings_usd
                .ok_or_else(|| BuildError::missing_field("total_earnings_usd"))?,
            total_volume_usd: self
                .total_volume_usd
                .ok_or_else(|| BuildError::missing_field("total_volume_usd"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            volume30d: self
                .volume30d
                .ok_or_else(|| BuildError::missing_field("volume30d"))?,
        })
    }
}
