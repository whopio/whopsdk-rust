pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReviewReport {
    /// When the report was filed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The report ID, prefixed `rrpt_`.
    #[serde(default)]
    pub id: String,
    /// The reason the reporter gave for reporting the review.
    #[serde(default)]
    pub reason: String,
    /// The ID of the review that was reported, prefixed `rev_`.
    #[serde(default)]
    pub review_id: String,
    /// Where the report stands. `pending` reports are waiting for Whop staff, `accepted` reports led to the review being removed, and `rejected` reports left the review up.
    pub status: ReviewReportStatus,
    /// When the report last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The ID of the user who filed the report, prefixed `user_`. Null if that user no longer exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ReviewReport {
    pub fn builder() -> ReviewReportBuilder {
        <ReviewReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewReportBuilder {
    created_at: Option<String>,
    id: Option<String>,
    reason: Option<String>,
    review_id: Option<String>,
    status: Option<ReviewReportStatus>,
    updated_at: Option<String>,
    user_id: Option<String>,
}

impl ReviewReportBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn review_id(mut self, value: impl Into<String>) -> Self {
        self.review_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ReviewReportStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewReport`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ReviewReportBuilder::created_at)
    /// - [`id`](ReviewReportBuilder::id)
    /// - [`reason`](ReviewReportBuilder::reason)
    /// - [`review_id`](ReviewReportBuilder::review_id)
    /// - [`status`](ReviewReportBuilder::status)
    /// - [`updated_at`](ReviewReportBuilder::updated_at)
    pub fn build(self) -> Result<ReviewReport, BuildError> {
        Ok(ReviewReport {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            review_id: self
                .review_id
                .ok_or_else(|| BuildError::missing_field("review_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self.user_id,
        })
    }
}
