pub use crate::prelude::*;

/// Query parameters for leaderboard
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardQueryRequest {
    /// Time window for the rankings. `day`, `month`, and `year` count earnings since the start of the current calendar day, month, or year; `last_30_days` counts earnings over the trailing 30 days; `all_time` ranks lifetime earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<LeaderboardPartnersRequestPeriod>,
}

impl LeaderboardQueryRequest {
    pub fn builder() -> LeaderboardQueryRequestBuilder {
        <LeaderboardQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardQueryRequestBuilder {
    period: Option<LeaderboardPartnersRequestPeriod>,
}

impl LeaderboardQueryRequestBuilder {
    pub fn period(mut self, value: LeaderboardPartnersRequestPeriod) -> Self {
        self.period = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardQueryRequest`].
    pub fn build(self) -> Result<LeaderboardQueryRequest, BuildError> {
        Ok(LeaderboardQueryRequest {
            period: self.period,
        })
    }
}
