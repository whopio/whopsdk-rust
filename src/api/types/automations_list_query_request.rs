pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationsListQueryRequest {
    /// Only this account's automations, prefixed `biz_`. Defaults to the credential's account, or every account a user can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only automations with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAutomationsRequestStatus>,
    /// Only automations this webhook event enrolls into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<ListAutomationsRequestTriggerEvent>,
    /// Field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAutomationsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAutomationsRequestDirection>,
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

impl AutomationsListQueryRequest {
    pub fn builder() -> AutomationsListQueryRequestBuilder {
        <AutomationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationsListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListAutomationsRequestStatus>,
    trigger_event: Option<ListAutomationsRequestTriggerEvent>,
    order: Option<ListAutomationsRequestOrder>,
    direction: Option<ListAutomationsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AutomationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListAutomationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trigger_event(mut self, value: ListAutomationsRequestTriggerEvent) -> Self {
        self.trigger_event = Some(value);
        self
    }

    pub fn order(mut self, value: ListAutomationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAutomationsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`AutomationsListQueryRequest`].
    pub fn build(self) -> Result<AutomationsListQueryRequest, BuildError> {
        Ok(AutomationsListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            trigger_event: self.trigger_event,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
