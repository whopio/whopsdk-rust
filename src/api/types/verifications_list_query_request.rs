pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerificationsListQueryRequest {
    /// Business account whose verifications you want to list, prefixed `biz_`. Provide this or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The caller's own user ID, prefixed `user_`, to list personal verifications. Provide this or `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Also cover the account's subaccounts. Requires `account_id`, and is a 400 without it. Lists the verifications of the account and every subaccount in account order, oldest account first, paging forward; `status`, `order`, `direction`, `last` and `before` are a 400. Leaves out removed subaccounts and ones you can't read verifications on. A page can hold fewer than `first` verifications while `has_next_page` is `true`; keep following `end_cursor`. On a subaccount, lists only that account's verifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<bool>,
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
    /// Field used to sort returned verifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVerificationsRequestOrder>,
    /// Sort direction for returned verifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListVerificationsRequestDirection>,
}

impl VerificationsListQueryRequest {
    pub fn builder() -> VerificationsListQueryRequestBuilder {
        <VerificationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    include_subaccounts: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListVerificationsRequestOrder>,
    direction: Option<ListVerificationsRequestDirection>,
}

impl VerificationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn include_subaccounts(mut self, value: bool) -> Self {
        self.include_subaccounts = Some(value);
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

    pub fn order(mut self, value: ListVerificationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListVerificationsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VerificationsListQueryRequest`].
    pub fn build(self) -> Result<VerificationsListQueryRequest, BuildError> {
        Ok(VerificationsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            include_subaccounts: self.include_subaccounts,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
