pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponse {
    /// The top referrers by total earnings, best first.
    #[serde(default)]
    pub leaders: Vec<LeaderboardPartnersResponseLeadersItem>,
    /// The caller's own standing; null when the caller has no referral earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub me: Option<LeaderboardPartnersResponseMe>,
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

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`leaders`](LeaderboardPartnersResponseBuilder::leaders)
    pub fn build(self) -> Result<LeaderboardPartnersResponse, BuildError> {
        Ok(LeaderboardPartnersResponse {
            leaders: self
                .leaders
                .ok_or_else(|| BuildError::missing_field("leaders"))?,
            me: self.me,
        })
    }
}
