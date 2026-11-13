pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentsListQueryRequest {
    /// Which sales to list. `account_sales` returns sales for the accounts the caller can read. `user_sales` returns only sales the signed-in user received personally, outside any account; it requires the user's own Whop login session and cannot be combined with `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ListPaymentsRequestMode>,
    /// Only payments charged by this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only payments in this lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPaymentsRequestStatus>,
    /// Only payments whose most recently attached kept shipment is in this tracking state. `missing_tracking` selects paid payments with a captured shipping address and no kept shipment, excluding fully refunded payments. `needs_tracking` selects payments with an open pre-shipment funds hold and no kept shipment. Shipment states do not imply a payment or refund state. Requires `shipment:basic:read` as well as `payment:basic:read`; user-wide lists include only accounts readable under both scopes. Only supported with `mode=account_sales`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_status: Option<ListPaymentsRequestTrackingStatus>,
    /// Only payments charged for this reason. `subscription_cycle` also matches renewals recorded as `subscription_update`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<ListPaymentsRequestBillingReason>,
    /// Only payments presented in this three-letter currency, such as `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Only payments made by this buyer, prefixed `user_`. This filters sales the caller can read, so `me` is not accepted; list the caller's own purchases with `GET /memberships?user_id=me`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Search payments by payment ID, dispute ID, user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Also cover the account's subaccounts. Requires `account_id`, and is a 400 without it. Without `query`, lists the payments of the account and every subaccount, newest first by `created_at` only; another `order` is a 400. The listing is cut off after 10 seconds, which a rare filter on an account with little recent activity can reach; add a `query` to narrow it. With `query`, matches on payment, dispute, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`); free-text queries such as a name search only the named account. Subaccounts' payments are returned only for subaccounts you can read payments on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<bool>,
    /// Only payments made by this member, prefixed `mber_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Only payments billed under this membership, prefixed `mem_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Only payments for this product, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Only payments priced by this variant, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
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
    /// Only payments in any of these lifecycle states, with the same meanings as `status`. Repeat the parameter to pass several. Combined with `status` when both are sent.
    #[serde(default)]
    pub statuses: Vec<Option<ListPaymentsRequestStatusesItem>>,
    /// Only payments whose `substatus` is any of these. Repeat the parameter to pass several. `incomplete` also matches payments that report `incomplete` because no finer status was recorded.
    #[serde(default)]
    pub substatuses: Vec<Option<ListPaymentsRequestSubstatusesItem>>,
    /// Only payments charged for any of these reasons, with the same meanings as `billing_reason`. Repeat the parameter to pass several. Combined with `billing_reason` when both are sent.
    #[serde(default)]
    pub billing_reasons: Vec<Option<ListPaymentsRequestBillingReasonsItem>>,
    /// Only payments presented in any of these three-letter currencies. Repeat the parameter to pass several. Combined with `currency` when both are sent.
    #[serde(default)]
    pub currencies: Vec<Option<String>>,
    /// Only payments charged at least this amount: what the buyer was charged after discounts and including tax, in the currency each payment was presented in, the same amount `order=amount` sorts by. Where tax is added on top of the price (`tax_behavior` is `exclusive`), that is the price plus the tax, so a 995 price with tax added does not match `995`: match a range instead. A failed payment that charged nothing matches 0, not its price. Inclusive and compared to the cent. Pair it with `currencies` when the account sells in several. Send the same value as `max_amount` to match one exact amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub min_amount: Option<f64>,
    /// Only payments charged at most this amount, measured like `min_amount`: after discounts and including tax, in the currency each payment was presented in. Inclusive and compared to the cent. Must not be less than `min_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub max_amount: Option<f64>,
    /// Only payments priced by any of these variants, each prefixed `plan_`. Repeat the parameter to pass several. Combined with `variant_id` when both are sent.
    #[serde(default)]
    pub variant_ids: Vec<Option<String>>,
    /// Only payments for any of these products, each prefixed `prod_`. Repeat the parameter to pass several. Combined with `product_id` when both are sent.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only payments paid with any of these methods: a card brand such as `amex`, a wallet such as `apple_pay`, or a payment method type such as `paypal`. Repeat the parameter to pass several.
    #[serde(default)]
    pub payment_methods: Vec<Option<ListPaymentsRequestPaymentMethodsItem>>,
    /// Only payments whose shipment is in any of these tracking states, or that still need a tracking number (`needs_tracking`). Repeat the parameter to pass several. Requires `account_id` and the shipment:basic:read permission.
    #[serde(default)]
    pub tracking_states: Vec<Option<ListPaymentsRequestTrackingStatesItem>>,
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
    /// The field to sort by. `amount` sorts by the amount charged, in the currency it was charged in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPaymentsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPaymentsRequestDirection>,
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
    /// Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
}

impl PaymentsListQueryRequest {
    pub fn builder() -> PaymentsListQueryRequestBuilder {
        <PaymentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentsListQueryRequestBuilder {
    mode: Option<ListPaymentsRequestMode>,
    account_id: Option<String>,
    status: Option<ListPaymentsRequestStatus>,
    tracking_status: Option<ListPaymentsRequestTrackingStatus>,
    billing_reason: Option<ListPaymentsRequestBillingReason>,
    currency: Option<String>,
    user_id: Option<String>,
    query: Option<String>,
    include_subaccounts: Option<bool>,
    member_id: Option<String>,
    membership_id: Option<String>,
    product_id: Option<String>,
    variant_id: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    statuses: Option<Vec<Option<ListPaymentsRequestStatusesItem>>>,
    substatuses: Option<Vec<Option<ListPaymentsRequestSubstatusesItem>>>,
    billing_reasons: Option<Vec<Option<ListPaymentsRequestBillingReasonsItem>>>,
    currencies: Option<Vec<Option<String>>>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    variant_ids: Option<Vec<Option<String>>>,
    product_ids: Option<Vec<Option<String>>>,
    payment_methods: Option<Vec<Option<ListPaymentsRequestPaymentMethodsItem>>>,
    tracking_states: Option<Vec<Option<ListPaymentsRequestTrackingStatesItem>>>,
    refunded_after: Option<DateTime<FixedOffset>>,
    refunded_before: Option<DateTime<FixedOffset>>,
    dispute_alerted_after: Option<DateTime<FixedOffset>>,
    dispute_alerted_before: Option<DateTime<FixedOffset>>,
    dispute_created_after: Option<DateTime<FixedOffset>>,
    dispute_created_before: Option<DateTime<FixedOffset>>,
    order: Option<ListPaymentsRequestOrder>,
    direction: Option<ListPaymentsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
}

impl PaymentsListQueryRequestBuilder {
    pub fn mode(mut self, value: ListPaymentsRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPaymentsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tracking_status(mut self, value: ListPaymentsRequestTrackingStatus) -> Self {
        self.tracking_status = Some(value);
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

    pub fn include_subaccounts(mut self, value: bool) -> Self {
        self.include_subaccounts = Some(value);
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

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
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

    pub fn statuses(mut self, value: Vec<Option<ListPaymentsRequestStatusesItem>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn substatuses(mut self, value: Vec<Option<ListPaymentsRequestSubstatusesItem>>) -> Self {
        self.substatuses = Some(value);
        self
    }

    pub fn billing_reasons(
        mut self,
        value: Vec<Option<ListPaymentsRequestBillingReasonsItem>>,
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
        value: Vec<Option<ListPaymentsRequestPaymentMethodsItem>>,
    ) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn tracking_states(
        mut self,
        value: Vec<Option<ListPaymentsRequestTrackingStatesItem>>,
    ) -> Self {
        self.tracking_states = Some(value);
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`statuses`](PaymentsListQueryRequestBuilder::statuses)
    /// - [`substatuses`](PaymentsListQueryRequestBuilder::substatuses)
    /// - [`billing_reasons`](PaymentsListQueryRequestBuilder::billing_reasons)
    /// - [`currencies`](PaymentsListQueryRequestBuilder::currencies)
    /// - [`variant_ids`](PaymentsListQueryRequestBuilder::variant_ids)
    /// - [`product_ids`](PaymentsListQueryRequestBuilder::product_ids)
    /// - [`payment_methods`](PaymentsListQueryRequestBuilder::payment_methods)
    /// - [`tracking_states`](PaymentsListQueryRequestBuilder::tracking_states)
    pub fn build(self) -> Result<PaymentsListQueryRequest, BuildError> {
        Ok(PaymentsListQueryRequest {
            mode: self.mode,
            account_id: self.account_id,
            status: self.status,
            tracking_status: self.tracking_status,
            billing_reason: self.billing_reason,
            currency: self.currency,
            user_id: self.user_id,
            query: self.query,
            include_subaccounts: self.include_subaccounts,
            member_id: self.member_id,
            membership_id: self.membership_id,
            product_id: self.product_id,
            variant_id: self.variant_id,
            created_before: self.created_before,
            created_after: self.created_after,
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
            refunded_after: self.refunded_after,
            refunded_before: self.refunded_before,
            dispute_alerted_after: self.dispute_alerted_after,
            dispute_alerted_before: self.dispute_alerted_before,
            dispute_created_after: self.dispute_created_after,
            dispute_created_before: self.dispute_created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
        })
    }
}
