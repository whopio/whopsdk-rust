pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefundsListQueryRequest {
    /// Only refunds issued by this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only refunds of this payment, prefixed `pay_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Only refunds to this buyer, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only refunds requested before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only refunds requested after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListRefundsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListRefundsRequestDirection>,
    /// The number of refunds to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns refunds after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of refunds to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns refunds before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl RefundsListQueryRequest {
    pub fn builder() -> RefundsListQueryRequestBuilder {
        <RefundsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundsListQueryRequestBuilder {
    account_id: Option<String>,
    payment_id: Option<String>,
    user_id: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    order: Option<ListRefundsRequestOrder>,
    direction: Option<ListRefundsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl RefundsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn order(mut self, value: ListRefundsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListRefundsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`RefundsListQueryRequest`].
    pub fn build(self) -> Result<RefundsListQueryRequest, BuildError> {
        Ok(RefundsListQueryRequest {
            account_id: self.account_id,
            payment_id: self.payment_id,
            user_id: self.user_id,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
