pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Payment {
    /// The account that received the payment, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// What the account keeps: the total less Whop's fees.
    #[serde(default)]
    pub amount_after_fees: Money,
    /// True when Whop refunded the payment automatically, for example on a dispute alert.
    #[serde(default)]
    pub auto_refunded: bool,
    /// The billing address the buyer entered, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<PaymentAddress>,
    /// Why the charge was created: a first purchase, a renewal, a one-time payment, or a manual charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<BillingReasons>,
    /// The checkout configuration the buyer paid through, prefixed `ch_`, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration_id: Option<String>,
    /// The credential a buyer's surface presents to poll this payment and set its return URL. Only on payments created from a confirmation token, and always null in list responses — retrieve the payment for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// When the payment was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The currency the payment settles in, lowercase ISO 4217. Every money field below is stated in it unless it says otherwise.
    pub currency: Currencies,
    /// The phone number the buyer gave at checkout, when one was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,
    /// The normalized decline reason of the most recent failed attempt, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<PaymentDeclineCodes>,
    /// When an issuer warned that this payment will be disputed, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_alerted_at: Option<String>,
    /// Why the most recent attempt failed, in plain words, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// For installment methods, how many payments the charge splits into.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub financing_installments_count: Option<f64>,
    /// Payment ID, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    /// When the most recent charge attempt ran, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_attempt_at: Option<String>,
    /// The buyer's member record on the account, prefixed `mber_`. Null without the member:basic:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The membership this payment is billed against, prefixed `mem_`. Null for one-off purchases or without the member:basic:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Your own key-value data attached when the payment was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// True when funds are held until the order ships and no tracking number has been added yet. Null without the shipment:basic:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_tracking: Option<bool>,
    /// When the next automatic retry is scheduled, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt_at: Option<String>,
    /// When the money was collected, or null while it has not been.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<String>,
    /// The instrument shaped for display: a buyer-facing name, the standard icon set, and the card's brand and last four when it was a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<PaymentInstrument>,
    /// The stored payment method that was charged, prefixed `payt_`. Null when the method was not saved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// The kind of instrument used, for example `card`, `apple_pay`, `klarna`, or `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodTypes>,
    /// How many charge attempts have failed on this payment.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub payments_failed: f64,
    /// The plan that was charged, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product the plan belongs to, prefixed `prod_`. Null for a plan with no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The promo code applied at checkout, prefixed `promo_`, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// True when the payment is `paid`, not yet fully refunded, and its processor supports refunds.
    #[serde(default)]
    pub refundable: bool,
    /// How much has been refunded so far, as it settled — refunds convert at the rate in force when each one was issued, not the payment's original rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_amount: Option<Money>,
    /// When the payment was refunded, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_at: Option<String>,
    /// True when the payment is `open` and Whop can attempt the charge again — see `POST /payments/{id}/retry`.
    #[serde(default)]
    pub retryable: bool,
    /// Whop's fraud risk score from 0 (lowest) to 100 (highest), or null when the payment was not scored.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub risk_score: Option<f64>,
    /// The factors behind `risk_score`, grouped by category, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_signals: Option<HashMap<String, serde_json::Value>>,
    /// When the funds post to the account's available balance, at midnight UTC. The `ledger_account.funds_available` webhook carries the same value. Null until the payment is paid, and always null in list responses — retrieve the payment for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_time_at: Option<String>,
    /// The shipment fulfilling this payment, prefixed `ship_`. Null when nothing ships or without the shipment:basic:read permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_id: Option<String>,
    /// The shipping address for physical goods, or null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<PaymentAddress>,
    /// The lifecycle state of the charge: `open` while collection is outstanding, `paid` once the money moved, `pending` while a settlement rail clears, `void`/`uncollectible` when it ended without collecting.
    pub status: ReceiptStatus,
    /// The dashboard's finer-grained reading of the payment, folding in refunds, disputes and Resolution Center cases.
    pub substatus: FriendlyReceiptStatus,
    /// The price before discounts, tax and fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<Money>,
    /// The sales tax or VAT collected. Null when no tax applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Money>,
    /// Whether `tax_amount` was added on top of the price (`exclusive`) or was already inside it (`inclusive`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<ReceiptTaxBehaviors>,
    /// How much of the collected tax has been returned to the buyer so far. Zero when the payment carried no tax, or when nothing has been refunded.
    #[serde(default)]
    pub tax_refunded_amount: Money,
    /// True when the buyer completed 3D Secure for this payment.
    #[serde(default)]
    pub three_ds_verified: bool,
    /// The account-facing total: the price after discounts, plus any tax added on top. Excludes buyer fees, which the buyer pays above this amount — so this is not necessarily what the buyer's statement shows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Money>,
    /// When the payment last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The total converted to USD at the time of the charge, for reporting across currencies. Excludes the adaptive pricing FX markup, which the account does not keep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd_total: Option<Money>,
    /// The buyer. Null when the payment belongs to a company buyer rather than a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSummary>,
    /// The issuer's address and security code check results, or null when the processor returned none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_checks: Option<PaymentVerificationChecks>,
    /// True when the payment is `open` on a past-due membership and its processor supports voiding — see `POST /payments/{id}/void`.
    #[serde(default)]
    pub voidable: bool,
}

impl Payment {
    pub fn builder() -> PaymentBuilder {
        <PaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentBuilder {
    account_id: Option<String>,
    amount_after_fees: Option<Money>,
    auto_refunded: Option<bool>,
    billing_address: Option<PaymentAddress>,
    billing_reason: Option<BillingReasons>,
    checkout_configuration_id: Option<String>,
    client_secret: Option<String>,
    created_at: Option<String>,
    currency: Option<Currencies>,
    customer_phone: Option<String>,
    decline_code: Option<PaymentDeclineCodes>,
    dispute_alerted_at: Option<String>,
    failure_message: Option<String>,
    financing_installments_count: Option<f64>,
    id: Option<String>,
    last_payment_attempt_at: Option<String>,
    member_id: Option<String>,
    membership_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    needs_tracking: Option<bool>,
    next_payment_attempt_at: Option<String>,
    paid_at: Option<String>,
    payment_instrument: Option<PaymentInstrument>,
    payment_method_id: Option<String>,
    payment_method_type: Option<PaymentMethodTypes>,
    payments_failed: Option<f64>,
    plan_id: Option<String>,
    product_id: Option<String>,
    promo_code_id: Option<String>,
    refundable: Option<bool>,
    refunded_amount: Option<Money>,
    refunded_at: Option<String>,
    retryable: Option<bool>,
    risk_score: Option<f64>,
    risk_signals: Option<HashMap<String, serde_json::Value>>,
    settlement_time_at: Option<String>,
    shipment_id: Option<String>,
    shipping_address: Option<PaymentAddress>,
    status: Option<ReceiptStatus>,
    substatus: Option<FriendlyReceiptStatus>,
    subtotal: Option<Money>,
    tax_amount: Option<Money>,
    tax_behavior: Option<ReceiptTaxBehaviors>,
    tax_refunded_amount: Option<Money>,
    three_ds_verified: Option<bool>,
    total: Option<Money>,
    updated_at: Option<String>,
    usd_total: Option<Money>,
    user: Option<UserSummary>,
    verification_checks: Option<PaymentVerificationChecks>,
    voidable: Option<bool>,
}

impl PaymentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount_after_fees(mut self, value: Money) -> Self {
        self.amount_after_fees = Some(value);
        self
    }

    pub fn auto_refunded(mut self, value: bool) -> Self {
        self.auto_refunded = Some(value);
        self
    }

    pub fn billing_address(mut self, value: PaymentAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn billing_reason(mut self, value: BillingReasons) -> Self {
        self.billing_reason = Some(value);
        self
    }

    pub fn checkout_configuration_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn customer_phone(mut self, value: impl Into<String>) -> Self {
        self.customer_phone = Some(value.into());
        self
    }

    pub fn decline_code(mut self, value: PaymentDeclineCodes) -> Self {
        self.decline_code = Some(value);
        self
    }

    pub fn dispute_alerted_at(mut self, value: impl Into<String>) -> Self {
        self.dispute_alerted_at = Some(value.into());
        self
    }

    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    pub fn financing_installments_count(mut self, value: f64) -> Self {
        self.financing_installments_count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_payment_attempt_at(mut self, value: impl Into<String>) -> Self {
        self.last_payment_attempt_at = Some(value.into());
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

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn needs_tracking(mut self, value: bool) -> Self {
        self.needs_tracking = Some(value);
        self
    }

    pub fn next_payment_attempt_at(mut self, value: impl Into<String>) -> Self {
        self.next_payment_attempt_at = Some(value.into());
        self
    }

    pub fn paid_at(mut self, value: impl Into<String>) -> Self {
        self.paid_at = Some(value.into());
        self
    }

    pub fn payment_instrument(mut self, value: PaymentInstrument) -> Self {
        self.payment_instrument = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    pub fn payments_failed(mut self, value: f64) -> Self {
        self.payments_failed = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn refundable(mut self, value: bool) -> Self {
        self.refundable = Some(value);
        self
    }

    pub fn refunded_amount(mut self, value: Money) -> Self {
        self.refunded_amount = Some(value);
        self
    }

    pub fn refunded_at(mut self, value: impl Into<String>) -> Self {
        self.refunded_at = Some(value.into());
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn risk_score(mut self, value: f64) -> Self {
        self.risk_score = Some(value);
        self
    }

    pub fn risk_signals(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.risk_signals = Some(value);
        self
    }

    pub fn settlement_time_at(mut self, value: impl Into<String>) -> Self {
        self.settlement_time_at = Some(value.into());
        self
    }

    pub fn shipment_id(mut self, value: impl Into<String>) -> Self {
        self.shipment_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: PaymentAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn status(mut self, value: ReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn substatus(mut self, value: FriendlyReceiptStatus) -> Self {
        self.substatus = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: Money) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn tax_behavior(mut self, value: ReceiptTaxBehaviors) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn tax_refunded_amount(mut self, value: Money) -> Self {
        self.tax_refunded_amount = Some(value);
        self
    }

    pub fn three_ds_verified(mut self, value: bool) -> Self {
        self.three_ds_verified = Some(value);
        self
    }

    pub fn total(mut self, value: Money) -> Self {
        self.total = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn usd_total(mut self, value: Money) -> Self {
        self.usd_total = Some(value);
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    pub fn verification_checks(mut self, value: PaymentVerificationChecks) -> Self {
        self.verification_checks = Some(value);
        self
    }

    pub fn voidable(mut self, value: bool) -> Self {
        self.voidable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Payment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_after_fees`](PaymentBuilder::amount_after_fees)
    /// - [`auto_refunded`](PaymentBuilder::auto_refunded)
    /// - [`created_at`](PaymentBuilder::created_at)
    /// - [`currency`](PaymentBuilder::currency)
    /// - [`id`](PaymentBuilder::id)
    /// - [`payments_failed`](PaymentBuilder::payments_failed)
    /// - [`refundable`](PaymentBuilder::refundable)
    /// - [`retryable`](PaymentBuilder::retryable)
    /// - [`status`](PaymentBuilder::status)
    /// - [`substatus`](PaymentBuilder::substatus)
    /// - [`tax_refunded_amount`](PaymentBuilder::tax_refunded_amount)
    /// - [`three_ds_verified`](PaymentBuilder::three_ds_verified)
    /// - [`updated_at`](PaymentBuilder::updated_at)
    /// - [`voidable`](PaymentBuilder::voidable)
    pub fn build(self) -> Result<Payment, BuildError> {
        Ok(Payment {
            account_id: self.account_id,
            amount_after_fees: self
                .amount_after_fees
                .ok_or_else(|| BuildError::missing_field("amount_after_fees"))?,
            auto_refunded: self
                .auto_refunded
                .ok_or_else(|| BuildError::missing_field("auto_refunded"))?,
            billing_address: self.billing_address,
            billing_reason: self.billing_reason,
            checkout_configuration_id: self.checkout_configuration_id,
            client_secret: self.client_secret,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_phone: self.customer_phone,
            decline_code: self.decline_code,
            dispute_alerted_at: self.dispute_alerted_at,
            failure_message: self.failure_message,
            financing_installments_count: self.financing_installments_count,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_payment_attempt_at: self.last_payment_attempt_at,
            member_id: self.member_id,
            membership_id: self.membership_id,
            metadata: self.metadata,
            needs_tracking: self.needs_tracking,
            next_payment_attempt_at: self.next_payment_attempt_at,
            paid_at: self.paid_at,
            payment_instrument: self.payment_instrument,
            payment_method_id: self.payment_method_id,
            payment_method_type: self.payment_method_type,
            payments_failed: self
                .payments_failed
                .ok_or_else(|| BuildError::missing_field("payments_failed"))?,
            plan_id: self.plan_id,
            product_id: self.product_id,
            promo_code_id: self.promo_code_id,
            refundable: self
                .refundable
                .ok_or_else(|| BuildError::missing_field("refundable"))?,
            refunded_amount: self.refunded_amount,
            refunded_at: self.refunded_at,
            retryable: self
                .retryable
                .ok_or_else(|| BuildError::missing_field("retryable"))?,
            risk_score: self.risk_score,
            risk_signals: self.risk_signals,
            settlement_time_at: self.settlement_time_at,
            shipment_id: self.shipment_id,
            shipping_address: self.shipping_address,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            substatus: self
                .substatus
                .ok_or_else(|| BuildError::missing_field("substatus"))?,
            subtotal: self.subtotal,
            tax_amount: self.tax_amount,
            tax_behavior: self.tax_behavior,
            tax_refunded_amount: self
                .tax_refunded_amount
                .ok_or_else(|| BuildError::missing_field("tax_refunded_amount"))?,
            three_ds_verified: self
                .three_ds_verified
                .ok_or_else(|| BuildError::missing_field("three_ds_verified"))?,
            total: self.total,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            usd_total: self.usd_total,
            user: self.user,
            verification_checks: self.verification_checks,
            voidable: self
                .voidable
                .ok_or_else(|| BuildError::missing_field("voidable"))?,
        })
    }
}
