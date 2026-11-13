pub use crate::prelude::*;

/// Query parameters for summary
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentsSummaryQueryRequest {
    /// Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    #[serde(default)]
    pub groups: Vec<Option<SummaryPaymentsRequestGroupsItem>>,
    /// Only payments charged by this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only payments in any of these lifecycle states. Repeat the parameter to pass several.
    #[serde(default)]
    pub statuses: Vec<Option<SummaryPaymentsRequestStatusesItem>>,
    /// Only payments whose `substatus` is any of these. Repeat the parameter to pass several.
    #[serde(default)]
    pub substatuses: Vec<Option<SummaryPaymentsRequestSubstatusesItem>>,
    /// Only payments charged for any of these reasons. Repeat the parameter to pass several.
    #[serde(default)]
    pub billing_reasons: Vec<Option<SummaryPaymentsRequestBillingReasonsItem>>,
    /// Only payments presented in any of these three-letter currencies. Repeat the parameter to pass several.
    #[serde(default)]
    pub currencies: Vec<Option<String>>,
    /// Only payments charged at least this amount: what the buyer was charged after discounts and including tax, in the currency each payment was presented in. Where tax is added on top of the price (`tax_behavior` is `exclusive`), that is the price plus the tax. A failed payment that charged nothing counts as 0. Inclusive and compared to the cent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_amount: Option<f64>,
    /// Only payments charged at most this amount, measured like `min_amount`: after discounts and including tax. Inclusive and compared to the cent. Must not be less than `min_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_amount: Option<f64>,
    /// Only payments priced by any of these variants, each prefixed `plan_`. Repeat the parameter to pass several.
    #[serde(default)]
    pub variant_ids: Vec<Option<String>>,
    /// Only payments for any of these products, each prefixed `prod_`. Repeat the parameter to pass several.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only payments paid with any of these methods. Repeat the parameter to pass several.
    #[serde(default)]
    pub payment_methods: Vec<Option<SummaryPaymentsRequestPaymentMethodsItem>>,
    /// Only payments in any of these tracking states. Requires `account_id` and the shipment:basic:read permission.
    #[serde(default)]
    pub tracking_states: Vec<Option<SummaryPaymentsRequestTrackingStatesItem>>,
    /// Only payments created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only payments created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only payments refunded after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub refunded_after: Option<DateTime<FixedOffset>>,
    /// Only payments refunded before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub refunded_before: Option<DateTime<FixedOffset>>,
    /// Only payments with a dispute alert after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_alerted_after: Option<DateTime<FixedOffset>>,
    /// Only payments with a dispute alert before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_alerted_before: Option<DateTime<FixedOffset>>,
    /// Only payments with a dispute opened after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_created_after: Option<DateTime<FixedOffset>>,
    /// Only payments with a dispute opened before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_created_before: Option<DateTime<FixedOffset>>,
    /// Search payments by payment ID, dispute ID, user ID, membership ID, user email, name, or username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only for searches: with `query`, also search the account's subaccounts. Requires `account_id` and `query`, and is a 400 without either; it never lists subaccounts' payments on its own. Matches on payment, dispute, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`). Free-text queries such as a name search only the named account. Subaccounts' payments are returned only for subaccounts you can read payments on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<bool>,
}

impl PaymentsSummaryQueryRequest {
    pub fn builder() -> PaymentsSummaryQueryRequestBuilder {
        <PaymentsSummaryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsSummaryQueryRequestBuilder {
    groups: Option<Vec<Option<SummaryPaymentsRequestGroupsItem>>>,
    account_id: Option<String>,
    statuses: Option<Vec<Option<SummaryPaymentsRequestStatusesItem>>>,
    substatuses: Option<Vec<Option<SummaryPaymentsRequestSubstatusesItem>>>,
    billing_reasons: Option<Vec<Option<SummaryPaymentsRequestBillingReasonsItem>>>,
    currencies: Option<Vec<Option<String>>>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    variant_ids: Option<Vec<Option<String>>>,
    product_ids: Option<Vec<Option<String>>>,
    payment_methods: Option<Vec<Option<SummaryPaymentsRequestPaymentMethodsItem>>>,
    tracking_states: Option<Vec<Option<SummaryPaymentsRequestTrackingStatesItem>>>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    refunded_after: Option<DateTime<FixedOffset>>,
    refunded_before: Option<DateTime<FixedOffset>>,
    dispute_alerted_after: Option<DateTime<FixedOffset>>,
    dispute_alerted_before: Option<DateTime<FixedOffset>>,
    dispute_created_after: Option<DateTime<FixedOffset>>,
    dispute_created_before: Option<DateTime<FixedOffset>>,
    query: Option<String>,
    include_subaccounts: Option<bool>,
}

impl PaymentsSummaryQueryRequestBuilder {
    pub fn groups(mut self, value: Vec<Option<SummaryPaymentsRequestGroupsItem>>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn statuses(mut self, value: Vec<Option<SummaryPaymentsRequestStatusesItem>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn substatuses(
        mut self,
        value: Vec<Option<SummaryPaymentsRequestSubstatusesItem>>,
    ) -> Self {
        self.substatuses = Some(value);
        self
    }

    pub fn billing_reasons(
        mut self,
        value: Vec<Option<SummaryPaymentsRequestBillingReasonsItem>>,
    ) -> Self {
        self.billing_reasons = Some(value);
        self
    }

    pub fn currencies(mut self, value: Vec<Option<String>>) -> Self {
        self.currencies = Some(value);
        self
    }

    pub fn min_amount(mut self, value: f64) -> Self {
        self.min_amount = Some(value);
        self
    }

    pub fn max_amount(mut self, value: f64) -> Self {
        self.max_amount = Some(value);
        self
    }

    pub fn variant_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn payment_methods(
        mut self,
        value: Vec<Option<SummaryPaymentsRequestPaymentMethodsItem>>,
    ) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn tracking_states(
        mut self,
        value: Vec<Option<SummaryPaymentsRequestTrackingStatesItem>>,
    ) -> Self {
        self.tracking_states = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn refunded_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.refunded_after = Some(value);
        self
    }

    pub fn refunded_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.refunded_before = Some(value);
        self
    }

    pub fn dispute_alerted_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_alerted_after = Some(value);
        self
    }

    pub fn dispute_alerted_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_alerted_before = Some(value);
        self
    }

    pub fn dispute_created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_created_after = Some(value);
        self
    }

    pub fn dispute_created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_created_before = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn include_subaccounts(mut self, value: bool) -> Self {
        self.include_subaccounts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsSummaryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](PaymentsSummaryQueryRequestBuilder::groups)
    /// - [`statuses`](PaymentsSummaryQueryRequestBuilder::statuses)
    /// - [`substatuses`](PaymentsSummaryQueryRequestBuilder::substatuses)
    /// - [`billing_reasons`](PaymentsSummaryQueryRequestBuilder::billing_reasons)
    /// - [`currencies`](PaymentsSummaryQueryRequestBuilder::currencies)
    /// - [`variant_ids`](PaymentsSummaryQueryRequestBuilder::variant_ids)
    /// - [`product_ids`](PaymentsSummaryQueryRequestBuilder::product_ids)
    /// - [`payment_methods`](PaymentsSummaryQueryRequestBuilder::payment_methods)
    /// - [`tracking_states`](PaymentsSummaryQueryRequestBuilder::tracking_states)
    pub fn build(self) -> Result<PaymentsSummaryQueryRequest, BuildError> {
        Ok(PaymentsSummaryQueryRequest {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            account_id: self.account_id,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            substatuses: self
                .substatuses
                .ok_or_else(|| BuildError::missing_field("substatuses"))?,
            billing_reasons: self
                .billing_reasons
                .ok_or_else(|| BuildError::missing_field("billing_reasons"))?,
            currencies: self
                .currencies
                .ok_or_else(|| BuildError::missing_field("currencies"))?,
            min_amount: self.min_amount,
            max_amount: self.max_amount,
            variant_ids: self
                .variant_ids
                .ok_or_else(|| BuildError::missing_field("variant_ids"))?,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            payment_methods: self
                .payment_methods
                .ok_or_else(|| BuildError::missing_field("payment_methods"))?,
            tracking_states: self
                .tracking_states
                .ok_or_else(|| BuildError::missing_field("tracking_states"))?,
            created_after: self.created_after,
            created_before: self.created_before,
            refunded_after: self.refunded_after,
            refunded_before: self.refunded_before,
            dispute_alerted_after: self.dispute_alerted_after,
            dispute_alerted_before: self.dispute_alerted_before,
            dispute_created_after: self.dispute_created_after,
            dispute_created_before: self.dispute_created_before,
            query: self.query,
            include_subaccounts: self.include_subaccounts,
        })
    }
}
