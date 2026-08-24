pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersRetrieveQueryRequest {
    /// When set, returns the user's account-specific profile overrides for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Also compute your balance history (opt-in; runs a heavier query). Only applies when the id is `me`; ignored for callers without balance-read scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_balance_history: Option<bool>,
    /// Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<RetrieveUsersRequestInterval>,
    /// IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl UsersRetrieveQueryRequest {
    pub fn builder() -> UsersRetrieveQueryRequestBuilder {
        <UsersRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersRetrieveQueryRequestBuilder {
    account_id: Option<String>,
    include_balance_history: Option<bool>,
    from: Option<String>,
    to: Option<String>,
    interval: Option<RetrieveUsersRequestInterval>,
    time_zone: Option<String>,
}

impl UsersRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn include_balance_history(mut self, value: bool) -> Self {
        self.include_balance_history = Some(value);
        self
    }

    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn interval(mut self, value: RetrieveUsersRequestInterval) -> Self {
        self.interval = Some(value);
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsersRetrieveQueryRequest`].
    pub fn build(self) -> Result<UsersRetrieveQueryRequest, BuildError> {
        Ok(UsersRetrieveQueryRequest {
            account_id: self.account_id,
            include_balance_history: self.include_balance_history,
            from: self.from,
            to: self.to,
            interval: self.interval,
            time_zone: self.time_zone,
        })
    }
}
