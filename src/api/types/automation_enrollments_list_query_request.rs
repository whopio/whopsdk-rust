pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationEnrollmentsListQueryRequest {
    /// Only this account's enrollments, prefixed `biz_`. Defaults to the credential's account, or every account a user can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only this automation's enrollments, prefixed `ato_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<String>,
    /// Only this member's enrollments, prefixed `mber_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Only enrollments with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAutomationEnrollmentsRequestStatus>,
    /// Only members on this automation version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAutomationEnrollmentsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAutomationEnrollmentsRequestDirection>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl AutomationEnrollmentsListQueryRequest {
    pub fn builder() -> AutomationEnrollmentsListQueryRequestBuilder {
        <AutomationEnrollmentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationEnrollmentsListQueryRequestBuilder {
    account_id: Option<String>,
    automation_id: Option<String>,
    member_id: Option<String>,
    status: Option<ListAutomationEnrollmentsRequestStatus>,
    version: Option<i64>,
    order: Option<ListAutomationEnrollmentsRequestOrder>,
    direction: Option<ListAutomationEnrollmentsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AutomationEnrollmentsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn automation_id(mut self, value: impl Into<String>) -> Self {
        self.automation_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListAutomationEnrollmentsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn order(mut self, value: ListAutomationEnrollmentsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAutomationEnrollmentsRequestDirection) -> Self {
        self.direction = Some(value);
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

    /// Consumes the builder and constructs a [`AutomationEnrollmentsListQueryRequest`].
    pub fn build(self) -> Result<AutomationEnrollmentsListQueryRequest, BuildError> {
        Ok(AutomationEnrollmentsListQueryRequest {
            account_id: self.account_id,
            automation_id: self.automation_id,
            member_id: self.member_id,
            status: self.status,
            version: self.version,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
