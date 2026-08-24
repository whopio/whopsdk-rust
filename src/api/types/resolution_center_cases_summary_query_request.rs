pub use crate::prelude::*;

/// Query parameters for summary
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCasesSummaryQueryRequest {
    /// Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    #[serde(default)]
    pub groups: Vec<Option<SummaryResolutionCenterCasesRequestGroupsItem>>,
    /// The account to summarize cases for (`biz_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only cases opened by this customer — a `user_` tag, or `me` for the calling user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only cases in these statuses.
    #[serde(default)]
    pub status: Vec<Option<SummaryResolutionCenterCasesRequestStatusItem>>,
    /// Only cases opened for these reasons.
    #[serde(default)]
    pub reason: Vec<Option<SummaryResolutionCenterCasesRequestReasonItem>>,
    /// Only closed cases that ended these ways.
    #[serde(default)]
    pub outcome: Vec<Option<SummaryResolutionCenterCasesRequestOutcomeItem>>,
    /// Only count cases created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only count cases created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl ResolutionCenterCasesSummaryQueryRequest {
    pub fn builder() -> ResolutionCenterCasesSummaryQueryRequestBuilder {
        <ResolutionCenterCasesSummaryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCasesSummaryQueryRequestBuilder {
    groups: Option<Vec<Option<SummaryResolutionCenterCasesRequestGroupsItem>>>,
    account_id: Option<String>,
    user_id: Option<String>,
    status: Option<Vec<Option<SummaryResolutionCenterCasesRequestStatusItem>>>,
    reason: Option<Vec<Option<SummaryResolutionCenterCasesRequestReasonItem>>>,
    outcome: Option<Vec<Option<SummaryResolutionCenterCasesRequestOutcomeItem>>>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl ResolutionCenterCasesSummaryQueryRequestBuilder {
    pub fn groups(
        mut self,
        value: Vec<Option<SummaryResolutionCenterCasesRequestGroupsItem>>,
    ) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: Vec<Option<SummaryResolutionCenterCasesRequestStatusItem>>,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reason(
        mut self,
        value: Vec<Option<SummaryResolutionCenterCasesRequestReasonItem>>,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn outcome(
        mut self,
        value: Vec<Option<SummaryResolutionCenterCasesRequestOutcomeItem>>,
    ) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCasesSummaryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](ResolutionCenterCasesSummaryQueryRequestBuilder::groups)
    /// - [`status`](ResolutionCenterCasesSummaryQueryRequestBuilder::status)
    /// - [`reason`](ResolutionCenterCasesSummaryQueryRequestBuilder::reason)
    /// - [`outcome`](ResolutionCenterCasesSummaryQueryRequestBuilder::outcome)
    pub fn build(self) -> Result<ResolutionCenterCasesSummaryQueryRequest, BuildError> {
        Ok(ResolutionCenterCasesSummaryQueryRequest {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            account_id: self.account_id,
            user_id: self.user_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            outcome: self
                .outcome
                .ok_or_else(|| BuildError::missing_field("outcome"))?,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
