pub use crate::prelude::*;

/// How many of the matching cases are in each status. Every status is present, including those with a count of zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryResolutionCenterCasesResponseGroupsStatus {
    #[serde(default)]
    pub awaiting_customer: i64,
    #[serde(default)]
    pub awaiting_merchant: i64,
    #[serde(default)]
    pub closed: i64,
    #[serde(default)]
    pub under_review: i64,
}

impl SummaryResolutionCenterCasesResponseGroupsStatus {
    pub fn builder() -> SummaryResolutionCenterCasesResponseGroupsStatusBuilder {
        <SummaryResolutionCenterCasesResponseGroupsStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryResolutionCenterCasesResponseGroupsStatusBuilder {
    awaiting_customer: Option<i64>,
    awaiting_merchant: Option<i64>,
    closed: Option<i64>,
    under_review: Option<i64>,
}

impl SummaryResolutionCenterCasesResponseGroupsStatusBuilder {
    pub fn awaiting_customer(mut self, value: i64) -> Self {
        self.awaiting_customer = Some(value);
        self
    }

    pub fn awaiting_merchant(mut self, value: i64) -> Self {
        self.awaiting_merchant = Some(value);
        self
    }

    pub fn closed(mut self, value: i64) -> Self {
        self.closed = Some(value);
        self
    }

    pub fn under_review(mut self, value: i64) -> Self {
        self.under_review = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryResolutionCenterCasesResponseGroupsStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`awaiting_customer`](SummaryResolutionCenterCasesResponseGroupsStatusBuilder::awaiting_customer)
    /// - [`awaiting_merchant`](SummaryResolutionCenterCasesResponseGroupsStatusBuilder::awaiting_merchant)
    /// - [`closed`](SummaryResolutionCenterCasesResponseGroupsStatusBuilder::closed)
    /// - [`under_review`](SummaryResolutionCenterCasesResponseGroupsStatusBuilder::under_review)
    pub fn build(self) -> Result<SummaryResolutionCenterCasesResponseGroupsStatus, BuildError> {
        Ok(SummaryResolutionCenterCasesResponseGroupsStatus {
            awaiting_customer: self
                .awaiting_customer
                .ok_or_else(|| BuildError::missing_field("awaiting_customer"))?,
            awaiting_merchant: self
                .awaiting_merchant
                .ok_or_else(|| BuildError::missing_field("awaiting_merchant"))?,
            closed: self
                .closed
                .ok_or_else(|| BuildError::missing_field("closed"))?,
            under_review: self
                .under_review
                .ok_or_else(|| BuildError::missing_field("under_review"))?,
        })
    }
}
