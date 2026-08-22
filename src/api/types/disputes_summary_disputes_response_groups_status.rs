pub use crate::prelude::*;

/// How many of the matching disputes are in each status. Every status is present, including those with a count of zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryDisputesResponseGroupsStatus {
    #[serde(default)]
    pub closed: i64,
    #[serde(default)]
    pub lost: i64,
    #[serde(default)]
    pub needs_response: i64,
    #[serde(default)]
    pub under_review: i64,
    #[serde(default)]
    pub won: i64,
}

impl SummaryDisputesResponseGroupsStatus {
    pub fn builder() -> SummaryDisputesResponseGroupsStatusBuilder {
        <SummaryDisputesResponseGroupsStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryDisputesResponseGroupsStatusBuilder {
    closed: Option<i64>,
    lost: Option<i64>,
    needs_response: Option<i64>,
    under_review: Option<i64>,
    won: Option<i64>,
}

impl SummaryDisputesResponseGroupsStatusBuilder {
    pub fn closed(mut self, value: i64) -> Self {
        self.closed = Some(value);
        self
    }

    pub fn lost(mut self, value: i64) -> Self {
        self.lost = Some(value);
        self
    }

    pub fn needs_response(mut self, value: i64) -> Self {
        self.needs_response = Some(value);
        self
    }

    pub fn under_review(mut self, value: i64) -> Self {
        self.under_review = Some(value);
        self
    }

    pub fn won(mut self, value: i64) -> Self {
        self.won = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryDisputesResponseGroupsStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`closed`](SummaryDisputesResponseGroupsStatusBuilder::closed)
    /// - [`lost`](SummaryDisputesResponseGroupsStatusBuilder::lost)
    /// - [`needs_response`](SummaryDisputesResponseGroupsStatusBuilder::needs_response)
    /// - [`under_review`](SummaryDisputesResponseGroupsStatusBuilder::under_review)
    /// - [`won`](SummaryDisputesResponseGroupsStatusBuilder::won)
    pub fn build(self) -> Result<SummaryDisputesResponseGroupsStatus, BuildError> {
        Ok(SummaryDisputesResponseGroupsStatus {
            closed: self
                .closed
                .ok_or_else(|| BuildError::missing_field("closed"))?,
            lost: self.lost.ok_or_else(|| BuildError::missing_field("lost"))?,
            needs_response: self
                .needs_response
                .ok_or_else(|| BuildError::missing_field("needs_response"))?,
            under_review: self
                .under_review
                .ok_or_else(|| BuildError::missing_field("under_review"))?,
            won: self.won.ok_or_else(|| BuildError::missing_field("won"))?,
        })
    }
}
