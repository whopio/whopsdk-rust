pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationStepsListQueryRequest {
    /// Only this automation's steps, in run order, prefixed `ato_`. `order` doesn't apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<String>,
    /// With `automation_id`, the version to list, such as its `published_version`. Defaults to its `draft_version`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// Without `automation_id`, only this account's steps, prefixed `biz_`. Defaults to the credential's account, or every account a user can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAutomationStepsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAutomationStepsRequestDirection>,
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

impl AutomationStepsListQueryRequest {
    pub fn builder() -> AutomationStepsListQueryRequestBuilder {
        <AutomationStepsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepsListQueryRequestBuilder {
    automation_id: Option<String>,
    version: Option<i64>,
    account_id: Option<String>,
    order: Option<ListAutomationStepsRequestOrder>,
    direction: Option<ListAutomationStepsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AutomationStepsListQueryRequestBuilder {
    pub fn automation_id(mut self, value: impl Into<String>) -> Self {
        self.automation_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAutomationStepsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAutomationStepsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`AutomationStepsListQueryRequest`].
    pub fn build(self) -> Result<AutomationStepsListQueryRequest, BuildError> {
        Ok(AutomationStepsListQueryRequest {
            automation_id: self.automation_id,
            version: self.version,
            account_id: self.account_id,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
