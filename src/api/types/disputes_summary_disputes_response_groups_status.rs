pub use crate::prelude::*;

/// How many of the matching disputes are in each status. Every status is present, including those with a count of zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryDisputesResponseGroupsStatus {
    #[serde(default)]
    pub generating_response: i64,
    #[serde(default)]
    pub lost: i64,
    #[serde(default)]
    pub needs_response: i64,
    #[serde(default)]
    pub prevented: i64,
    #[serde(default)]
    pub under_review: i64,
    #[serde(default)]
    pub warning_closed: i64,
    #[serde(default)]
    pub warning_needs_response: i64,
    #[serde(default)]
    pub warning_under_review: i64,
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
    generating_response: Option<i64>,
    lost: Option<i64>,
    needs_response: Option<i64>,
    prevented: Option<i64>,
    under_review: Option<i64>,
    warning_closed: Option<i64>,
    warning_needs_response: Option<i64>,
    warning_under_review: Option<i64>,
    won: Option<i64>,
}

impl SummaryDisputesResponseGroupsStatusBuilder {
    pub fn generating_response(mut self, value: i64) -> Self {
        self.generating_response = Some(value);
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

    pub fn prevented(mut self, value: i64) -> Self {
        self.prevented = Some(value);
        self
    }

    pub fn under_review(mut self, value: i64) -> Self {
        self.under_review = Some(value);
        self
    }

    pub fn warning_closed(mut self, value: i64) -> Self {
        self.warning_closed = Some(value);
        self
    }

    pub fn warning_needs_response(mut self, value: i64) -> Self {
        self.warning_needs_response = Some(value);
        self
    }

    pub fn warning_under_review(mut self, value: i64) -> Self {
        self.warning_under_review = Some(value);
        self
    }

    pub fn won(mut self, value: i64) -> Self {
        self.won = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryDisputesResponseGroupsStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generating_response`](SummaryDisputesResponseGroupsStatusBuilder::generating_response)
    /// - [`lost`](SummaryDisputesResponseGroupsStatusBuilder::lost)
    /// - [`needs_response`](SummaryDisputesResponseGroupsStatusBuilder::needs_response)
    /// - [`prevented`](SummaryDisputesResponseGroupsStatusBuilder::prevented)
    /// - [`under_review`](SummaryDisputesResponseGroupsStatusBuilder::under_review)
    /// - [`warning_closed`](SummaryDisputesResponseGroupsStatusBuilder::warning_closed)
    /// - [`warning_needs_response`](SummaryDisputesResponseGroupsStatusBuilder::warning_needs_response)
    /// - [`warning_under_review`](SummaryDisputesResponseGroupsStatusBuilder::warning_under_review)
    /// - [`won`](SummaryDisputesResponseGroupsStatusBuilder::won)
    pub fn build(self) -> Result<SummaryDisputesResponseGroupsStatus, BuildError> {
        Ok(SummaryDisputesResponseGroupsStatus {
            generating_response: self
                .generating_response
                .ok_or_else(|| BuildError::missing_field("generating_response"))?,
            lost: self.lost.ok_or_else(|| BuildError::missing_field("lost"))?,
            needs_response: self
                .needs_response
                .ok_or_else(|| BuildError::missing_field("needs_response"))?,
            prevented: self
                .prevented
                .ok_or_else(|| BuildError::missing_field("prevented"))?,
            under_review: self
                .under_review
                .ok_or_else(|| BuildError::missing_field("under_review"))?,
            warning_closed: self
                .warning_closed
                .ok_or_else(|| BuildError::missing_field("warning_closed"))?,
            warning_needs_response: self
                .warning_needs_response
                .ok_or_else(|| BuildError::missing_field("warning_needs_response"))?,
            warning_under_review: self
                .warning_under_review
                .ok_or_else(|| BuildError::missing_field("warning_under_review"))?,
            won: self.won.ok_or_else(|| BuildError::missing_field("won"))?,
        })
    }
}
