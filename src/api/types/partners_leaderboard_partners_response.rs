pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponse {
    /// The top 10 referrers by total earnings, best first.
    #[serde(default)]
    pub leaders: Vec<LeaderboardPartnersResponseLeadersItem>,
    /// The caller's own standing; null when the caller has no referral earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<LeaderboardPartnersResponseMe>,
    /// The referrers ranked within five places of the caller, including the caller, best first. Empty when the caller has no referral earnings or is anonymous.
    #[serde(default)]
    pub nearby: Vec<LeaderboardPartnersResponseNearbyItem>,
}

impl LeaderboardPartnersResponse {
    pub fn builder() -> LeaderboardPartnersResponseBuilder {
        <LeaderboardPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseBuilder {
    leaders: Option<Vec<LeaderboardPartnersResponseLeadersItem>>,
    me: Option<LeaderboardPartnersResponseMe>,
    nearby: Option<Vec<LeaderboardPartnersResponseNearbyItem>>,
}

impl LeaderboardPartnersResponseBuilder {
    pub fn leaders(mut self, value: Vec<LeaderboardPartnersResponseLeadersItem>) -> Self {
        self.leaders = Some(value);
        self
    }

    pub fn me(mut self, value: LeaderboardPartnersResponseMe) -> Self {
        self.me = Some(value);
        self
    }

    pub fn nearby(mut self, value: Vec<LeaderboardPartnersResponseNearbyItem>) -> Self {
        self.nearby = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`leaders`](LeaderboardPartnersResponseBuilder::leaders)
    /// - [`nearby`](LeaderboardPartnersResponseBuilder::nearby)
    pub fn build(self) -> Result<LeaderboardPartnersResponse, BuildError> {
        Ok(LeaderboardPartnersResponse {
            leaders: self
                .leaders
                .ok_or_else(|| BuildError::missing_field("leaders"))?,
            me: self.me,
            nearby: self
                .nearby
                .ok_or_else(|| BuildError::missing_field("nearby"))?,
        })
    }
}
