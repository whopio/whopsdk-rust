pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentRulesListQueryRequest {
    /// Only return rules belonging to this account. Defaults to the account the request is acting for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return rules with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPaymentRulesRequestStatus>,
    /// Only return rules that take this action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ListPaymentRulesRequestAction>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPaymentRulesRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPaymentRulesRequestDirection>,
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

impl PaymentRulesListQueryRequest {
    pub fn builder() -> PaymentRulesListQueryRequestBuilder {
        <PaymentRulesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRulesListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListPaymentRulesRequestStatus>,
    action: Option<ListPaymentRulesRequestAction>,
    order: Option<ListPaymentRulesRequestOrder>,
    direction: Option<ListPaymentRulesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PaymentRulesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPaymentRulesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn action(mut self, value: ListPaymentRulesRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn order(mut self, value: ListPaymentRulesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPaymentRulesRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`PaymentRulesListQueryRequest`].
    pub fn build(self) -> Result<PaymentRulesListQueryRequest, BuildError> {
        Ok(PaymentRulesListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            action: self.action,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
