pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCasesListQueryRequest {
    /// Only cases filed against this account (`biz_` tag). With read access to the account this lists its whole queue; without, only the cases you opened against it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only cases opened by this customer — a `user_` tag, or `me` for the calling user. It narrows what you can already read, so `me` lists the cases you opened without the ones on accounts you are a team member of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The number of cases to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns cases after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of cases to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns cases before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort cases by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListResolutionCenterCasesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListResolutionCenterCasesRequestDirection>,
    /// Only cases in these statuses. Repeat the parameter to pass several — one paginated list covers all of them.
    #[serde(default)]
    pub status: Vec<Option<ListResolutionCenterCasesRequestStatusItem>>,
    /// Only cases opened for these reasons. Repeat the parameter to pass several.
    #[serde(default)]
    pub reason: Vec<Option<ListResolutionCenterCasesRequestReasonItem>>,
    /// Only closed cases that ended these ways. Repeat the parameter to pass several.
    #[serde(default)]
    pub outcome: Vec<Option<ListResolutionCenterCasesRequestOutcomeItem>>,
    /// Only cases created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only cases created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl ResolutionCenterCasesListQueryRequest {
    pub fn builder() -> ResolutionCenterCasesListQueryRequestBuilder {
        <ResolutionCenterCasesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCasesListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListResolutionCenterCasesRequestOrder>,
    direction: Option<ListResolutionCenterCasesRequestDirection>,
    status: Option<Vec<Option<ListResolutionCenterCasesRequestStatusItem>>>,
    reason: Option<Vec<Option<ListResolutionCenterCasesRequestReasonItem>>>,
    outcome: Option<Vec<Option<ListResolutionCenterCasesRequestOutcomeItem>>>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl ResolutionCenterCasesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListResolutionCenterCasesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListResolutionCenterCasesRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: Vec<Option<ListResolutionCenterCasesRequestStatusItem>>,
    ) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reason(
        mut self,
        value: Vec<Option<ListResolutionCenterCasesRequestReasonItem>>,
    ) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn outcome(
        mut self,
        value: Vec<Option<ListResolutionCenterCasesRequestOutcomeItem>>,
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

    /// Consumes the builder and constructs a [`ResolutionCenterCasesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](ResolutionCenterCasesListQueryRequestBuilder::status)
    /// - [`reason`](ResolutionCenterCasesListQueryRequestBuilder::reason)
    /// - [`outcome`](ResolutionCenterCasesListQueryRequestBuilder::outcome)
    pub fn build(self) -> Result<ResolutionCenterCasesListQueryRequest, BuildError> {
        Ok(ResolutionCenterCasesListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
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
