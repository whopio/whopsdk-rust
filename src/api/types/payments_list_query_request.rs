pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the company to list payments for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ReceiptV2Order>,
    /// Filter payments to only those associated with these specific product identifiers.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Filter payments by their billing reason.
    #[serde(default)]
    pub billing_reasons: Vec<Option<BillingReasons>>,
    /// Filter payments by their currency code.
    #[serde(default)]
    pub currencies: Vec<Option<Currencies>>,
    /// Filter payments to only those associated with these specific plan identifiers.
    #[serde(default)]
    pub plan_ids: Vec<Option<String>>,
    /// Filter payments by their current status.
    #[serde(default)]
    pub statuses: Vec<Option<ReceiptStatus>>,
    /// Filter payments by their current substatus for more granular filtering.
    #[serde(default)]
    pub substatuses: Vec<Option<FriendlyReceiptStatus>>,
    /// Whether to include payments with a zero amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_free: Option<bool>,
    /// Only return payments created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return payments created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only return payments last updated before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_before: Option<DateTime<FixedOffset>>,
    /// Only return payments last updated after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_after: Option<DateTime<FixedOffset>>,
    /// Search payments by user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return payments from these checkout configurations.
    #[serde(default)]
    pub checkout_configuration_ids: Vec<Option<String>>,
}

impl PaymentsListQueryRequest {
    pub fn builder() -> PaymentsListQueryRequestBuilder {
        <PaymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    company_id: Option<String>,
    direction: Option<Direction>,
    order: Option<ReceiptV2Order>,
    product_ids: Option<Vec<Option<String>>>,
    billing_reasons: Option<Vec<Option<BillingReasons>>>,
    currencies: Option<Vec<Option<Currencies>>>,
    plan_ids: Option<Vec<Option<String>>>,
    statuses: Option<Vec<Option<ReceiptStatus>>>,
    substatuses: Option<Vec<Option<FriendlyReceiptStatus>>>,
    include_free: Option<bool>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    updated_before: Option<DateTime<FixedOffset>>,
    updated_after: Option<DateTime<FixedOffset>>,
    query: Option<String>,
    checkout_configuration_ids: Option<Vec<Option<String>>>,
}

impl PaymentsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: ReceiptV2Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn billing_reasons(mut self, value: Vec<Option<BillingReasons>>) -> Self {
        self.billing_reasons = Some(value);
        self
    }

    pub fn currencies(mut self, value: Vec<Option<Currencies>>) -> Self {
        self.currencies = Some(value);
        self
    }

    pub fn plan_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.plan_ids = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<Option<ReceiptStatus>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn substatuses(mut self, value: Vec<Option<FriendlyReceiptStatus>>) -> Self {
        self.substatuses = Some(value);
        self
    }

    pub fn include_free(mut self, value: bool) -> Self {
        self.include_free = Some(value);
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

    pub fn updated_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_before = Some(value);
        self
    }

    pub fn updated_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_after = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn checkout_configuration_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.checkout_configuration_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_ids`](PaymentsListQueryRequestBuilder::product_ids)
    /// - [`billing_reasons`](PaymentsListQueryRequestBuilder::billing_reasons)
    /// - [`currencies`](PaymentsListQueryRequestBuilder::currencies)
    /// - [`plan_ids`](PaymentsListQueryRequestBuilder::plan_ids)
    /// - [`statuses`](PaymentsListQueryRequestBuilder::statuses)
    /// - [`substatuses`](PaymentsListQueryRequestBuilder::substatuses)
    /// - [`checkout_configuration_ids`](PaymentsListQueryRequestBuilder::checkout_configuration_ids)
    pub fn build(self) -> Result<PaymentsListQueryRequest, BuildError> {
        Ok(PaymentsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            company_id: self.company_id,
            direction: self.direction,
            order: self.order,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            billing_reasons: self
                .billing_reasons
                .ok_or_else(|| BuildError::missing_field("billing_reasons"))?,
            currencies: self
                .currencies
                .ok_or_else(|| BuildError::missing_field("currencies"))?,
            plan_ids: self
                .plan_ids
                .ok_or_else(|| BuildError::missing_field("plan_ids"))?,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            substatuses: self
                .substatuses
                .ok_or_else(|| BuildError::missing_field("substatuses"))?,
            include_free: self.include_free,
            created_before: self.created_before,
            created_after: self.created_after,
            updated_before: self.updated_before,
            updated_after: self.updated_after,
            query: self.query,
            checkout_configuration_ids: self
                .checkout_configuration_ids
                .ok_or_else(|| BuildError::missing_field("checkout_configuration_ids"))?,
        })
    }
}
