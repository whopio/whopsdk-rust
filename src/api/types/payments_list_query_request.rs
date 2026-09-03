pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsListQueryRequest {
    /// Only payments charged by this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only payments in this lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPaymentsRequestStatus>,
    /// Only payments charged for this reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<ListPaymentsRequestBillingReason>,
    /// Only payments presented in this three-letter currency, such as `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Only payments made by this buyer, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Search payments by user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only payments made by this member, prefixed `mber_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Only payments billed under this membership, prefixed `mem_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Only payments for this product, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Only payments priced by this plan, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Only payments created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only payments created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPaymentsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPaymentsRequestDirection>,
    /// The number of payments to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns payments after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of payments to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns payments before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PaymentsListQueryRequest {
    pub fn builder() -> PaymentsListQueryRequestBuilder {
        <PaymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListPaymentsRequestStatus>,
    billing_reason: Option<ListPaymentsRequestBillingReason>,
    currency: Option<String>,
    user_id: Option<String>,
    query: Option<String>,
    member_id: Option<String>,
    membership_id: Option<String>,
    product_id: Option<String>,
    plan_id: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    order: Option<ListPaymentsRequestOrder>,
    direction: Option<ListPaymentsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PaymentsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPaymentsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn billing_reason(mut self, value: ListPaymentsRequestBillingReason) -> Self {
        self.billing_reason = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
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

    pub fn order(mut self, value: ListPaymentsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPaymentsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`PaymentsListQueryRequest`].
    pub fn build(self) -> Result<PaymentsListQueryRequest, BuildError> {
        Ok(PaymentsListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            billing_reason: self.billing_reason,
            currency: self.currency,
            user_id: self.user_id,
            query: self.query,
            member_id: self.member_id,
            membership_id: self.membership_id,
            product_id: self.product_id,
            plan_id: self.plan_id,
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
