pub use crate::prelude::*;

/// A payment represents a completed or attempted charge. Payments track the amount, status, currency, and payment method used.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentListItem {
    /// How much the payment is for after fees
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_after_fees: f64,
    /// The application fee charged on this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<PaymentListItemApplicationFee>,
    /// Whether this payment was auto refunded or not
    #[serde(default)]
    pub auto_refunded: bool,
    /// The address of the user who made the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<PaymentListItemBillingAddress>,
    /// The machine-readable reason this charge was created, such as initial subscription purchase, renewal cycle, or one-time payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<BillingReasons>,
    /// Card network reported by the processor (e.g., 'visa', 'mastercard', 'amex'). Present only when the payment method type is 'card'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<CardBrands>,
    /// The last four digits of the card used to make this payment. Null if the payment was not made with a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    /// The ID of the checkout session/configuration that produced this payment, if any. Use this to map payments back to the checkout configuration that created them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration_id: Option<String>,
    /// The company for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<PaymentListItemCompany>,
    /// The datetime the payment was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The three-letter ISO currency code for this payment (e.g., 'usd', 'eur').
    pub currency: Currencies,
    /// Phone number the customer provided at checkout, or their verified phone number when your checkout requires phone verification. `null` when no phone number was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,
    /// The reason the payment was declined. Null if the payment did not fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<PaymentDeclineCodes>,
    /// When an alert came in that this transaction will be disputed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub dispute_alerted_at: Option<DateTime<FixedOffset>>,
    /// If the payment failed, the reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
    /// The time of the last payment attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_payment_attempt: Option<DateTime<FixedOffset>>,
    /// The member attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PaymentListItemMember>,
    /// The membership attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership: Option<PaymentListItemMembership>,
    /// The custom metadata stored on this payment. This will be copied over to the checkout configuration for which this payment was made
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Whether this payment is holding funds until the order ships and has no tracking number yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_tracking: Option<bool>,
    /// The time of the next schedule payment retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_payment_attempt: Option<DateTime<FixedOffset>>,
    /// The time at which this payment was successfully collected. Null if the payment has not yet succeeded. As a Unix timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub paid_at: Option<DateTime<FixedOffset>>,
    /// The instrument this payment was made with, shaped for display: the method type, a buyer-facing name, the standard icon set, and the card facts when it was a card. Null when the receipt names no payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<PaymentListItemPaymentInstrument>,
    /// The tokenized payment method reference used for this payment. Null if no token was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentListItemPaymentMethod>,
    /// The type of payment instrument used for this payment (e.g., card, Cash App, iDEAL, Klarna, crypto). Null when the processor does not supply a type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodTypes>,
    /// The number of failed payment attempts for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_failed: Option<i64>,
    /// The plan attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PaymentListItemPlan>,
    /// The product this payment was made for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<PaymentListItemProduct>,
    /// The promo code used for this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<PaymentListItemPromoCode>,
    /// True only for payments that are `paid`, have not been fully refunded, and were processed by a payment processor that allows refunds.
    #[serde(default)]
    pub refundable: bool,
    /// The payment refund amount(if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub refunded_amount: Option<f64>,
    /// When the payment was refunded (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub refunded_at: Option<DateTime<FixedOffset>>,
    /// True when the payment status is `open` and its membership is in one of the retry-eligible states (`active`, `trialing`, `completed`, or `past_due`), or when it is a failed initial billing-engine payment on a `drafted` membership with an unlimited-stock plan; otherwise false. Used to decide if Whop can attempt the charge again.
    #[serde(default)]
    pub retryable: bool,
    /// The three-letter ISO currency code for this payment (e.g., 'usd', 'eur').
    pub settlement_currency: Currencies,
    /// The shipment attached to this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment: Option<PaymentListItemShipment>,
    /// The shipping address provided by the customer for physical goods. Null if no shipping address was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<PaymentListItemShippingAddress>,
    /// The current lifecycle state of this payment (e.g., 'draft', 'open', 'paid', 'void').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiptStatus>,
    /// The friendly status of the payment.
    pub substatus: FriendlyReceiptStatus,
    /// The subtotal to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub subtotal: Option<f64>,
    /// The calculated amount of the sales/VAT tax (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tax_amount: Option<f64>,
    /// The type of tax inclusivity applied to the payment, for determining whether the tax is included in the final price, or paid on top.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<ReceiptTaxBehaviors>,
    /// The total to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total: Option<f64>,
    /// The datetime the payment was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The total in USD to show to the creator (excluding buyer fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub usd_total: Option<f64>,
    /// The user that made this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<PaymentListItemUser>,
    /// True when the payment is tied to a membership in `past_due`, the payment status is `open`, and the processor allows voiding payments; otherwise false.
    #[serde(default)]
    pub voidable: bool,
}

impl PaymentListItem {
    pub fn builder() -> PaymentListItemBuilder {
        <PaymentListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemBuilder {
    amount_after_fees: Option<f64>,
    application_fee: Option<PaymentListItemApplicationFee>,
    auto_refunded: Option<bool>,
    billing_address: Option<PaymentListItemBillingAddress>,
    billing_reason: Option<BillingReasons>,
    card_brand: Option<CardBrands>,
    card_last4: Option<String>,
    checkout_configuration_id: Option<String>,
    company: Option<PaymentListItemCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    customer_phone: Option<String>,
    decline_code: Option<PaymentDeclineCodes>,
    dispute_alerted_at: Option<DateTime<FixedOffset>>,
    failure_message: Option<String>,
    id: Option<String>,
    last_payment_attempt: Option<DateTime<FixedOffset>>,
    member: Option<PaymentListItemMember>,
    membership: Option<PaymentListItemMembership>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    needs_tracking: Option<bool>,
    next_payment_attempt: Option<DateTime<FixedOffset>>,
    paid_at: Option<DateTime<FixedOffset>>,
    payment_instrument: Option<PaymentListItemPaymentInstrument>,
    payment_method: Option<PaymentListItemPaymentMethod>,
    payment_method_type: Option<PaymentMethodTypes>,
    payments_failed: Option<i64>,
    plan: Option<PaymentListItemPlan>,
    product: Option<PaymentListItemProduct>,
    promo_code: Option<PaymentListItemPromoCode>,
    refundable: Option<bool>,
    refunded_amount: Option<f64>,
    refunded_at: Option<DateTime<FixedOffset>>,
    retryable: Option<bool>,
    settlement_currency: Option<Currencies>,
    shipment: Option<PaymentListItemShipment>,
    shipping_address: Option<PaymentListItemShippingAddress>,
    status: Option<ReceiptStatus>,
    substatus: Option<FriendlyReceiptStatus>,
    subtotal: Option<f64>,
    tax_amount: Option<f64>,
    tax_behavior: Option<ReceiptTaxBehaviors>,
    total: Option<f64>,
    updated_at: Option<DateTime<FixedOffset>>,
    usd_total: Option<f64>,
    user: Option<PaymentListItemUser>,
    voidable: Option<bool>,
}

impl PaymentListItemBuilder {
    pub fn amount_after_fees(mut self, value: f64) -> Self {
        self.amount_after_fees = Some(value);
        self
    }

    pub fn application_fee(mut self, value: PaymentListItemApplicationFee) -> Self {
        self.application_fee = Some(value);
        self
    }

    pub fn auto_refunded(mut self, value: bool) -> Self {
        self.auto_refunded = Some(value);
        self
    }

    pub fn billing_address(mut self, value: PaymentListItemBillingAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn billing_reason(mut self, value: BillingReasons) -> Self {
        self.billing_reason = Some(value);
        self
    }

    pub fn card_brand(mut self, value: CardBrands) -> Self {
        self.card_brand = Some(value);
        self
    }

    pub fn card_last4(mut self, value: impl Into<String>) -> Self {
        self.card_last4 = Some(value.into());
        self
    }

    pub fn checkout_configuration_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration_id = Some(value.into());
        self
    }

    pub fn company(mut self, value: PaymentListItemCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn dispute_alerted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.dispute_alerted_at = Some(value);
        self
    }

    pub fn failure_message(mut self, value: impl Into<String>) -> Self {
        self.failure_message = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_payment_attempt(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_payment_attempt = Some(value);
        self
    }

    pub fn member(mut self, value: PaymentListItemMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn membership(mut self, value: PaymentListItemMembership) -> Self {
        self.membership = Some(value);
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

    pub fn next_payment_attempt(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_payment_attempt = Some(value);
        self
    }

    pub fn paid_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.paid_at = Some(value);
        self
    }

    pub fn payment_instrument(mut self, value: PaymentListItemPaymentInstrument) -> Self {
        self.payment_instrument = Some(value);
        self
    }

    pub fn payment_method(mut self, value: PaymentListItemPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    pub fn payments_failed(mut self, value: i64) -> Self {
        self.payments_failed = Some(value);
        self
    }

    pub fn plan(mut self, value: PaymentListItemPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: PaymentListItemProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn promo_code(mut self, value: PaymentListItemPromoCode) -> Self {
        self.promo_code = Some(value);
        self
    }

    pub fn refundable(mut self, value: bool) -> Self {
        self.refundable = Some(value);
        self
    }

    pub fn refunded_amount(mut self, value: f64) -> Self {
        self.refunded_amount = Some(value);
        self
    }

    pub fn refunded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.refunded_at = Some(value);
        self
    }

    pub fn retryable(mut self, value: bool) -> Self {
        self.retryable = Some(value);
        self
    }

    pub fn settlement_currency(mut self, value: Currencies) -> Self {
        self.settlement_currency = Some(value);
        self
    }

    pub fn shipment(mut self, value: PaymentListItemShipment) -> Self {
        self.shipment = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: PaymentListItemShippingAddress) -> Self {
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

    pub fn subtotal(mut self, value: f64) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: f64) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn tax_behavior(mut self, value: ReceiptTaxBehaviors) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn usd_total(mut self, value: f64) -> Self {
        self.usd_total = Some(value);
        self
    }

    pub fn user(mut self, value: PaymentListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn voidable(mut self, value: bool) -> Self {
        self.voidable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_after_fees`](PaymentListItemBuilder::amount_after_fees)
    /// - [`auto_refunded`](PaymentListItemBuilder::auto_refunded)
    /// - [`created_at`](PaymentListItemBuilder::created_at)
    /// - [`currency`](PaymentListItemBuilder::currency)
    /// - [`id`](PaymentListItemBuilder::id)
    /// - [`refundable`](PaymentListItemBuilder::refundable)
    /// - [`retryable`](PaymentListItemBuilder::retryable)
    /// - [`settlement_currency`](PaymentListItemBuilder::settlement_currency)
    /// - [`substatus`](PaymentListItemBuilder::substatus)
    /// - [`updated_at`](PaymentListItemBuilder::updated_at)
    /// - [`voidable`](PaymentListItemBuilder::voidable)
    pub fn build(self) -> Result<PaymentListItem, BuildError> {
        Ok(PaymentListItem {
            amount_after_fees: self
                .amount_after_fees
                .ok_or_else(|| BuildError::missing_field("amount_after_fees"))?,
            application_fee: self.application_fee,
            auto_refunded: self
                .auto_refunded
                .ok_or_else(|| BuildError::missing_field("auto_refunded"))?,
            billing_address: self.billing_address,
            billing_reason: self.billing_reason,
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            checkout_configuration_id: self.checkout_configuration_id,
            company: self.company,
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
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_payment_attempt: self.last_payment_attempt,
            member: self.member,
            membership: self.membership,
            metadata: self.metadata,
            needs_tracking: self.needs_tracking,
            next_payment_attempt: self.next_payment_attempt,
            paid_at: self.paid_at,
            payment_instrument: self.payment_instrument,
            payment_method: self.payment_method,
            payment_method_type: self.payment_method_type,
            payments_failed: self.payments_failed,
            plan: self.plan,
            product: self.product,
            promo_code: self.promo_code,
            refundable: self
                .refundable
                .ok_or_else(|| BuildError::missing_field("refundable"))?,
            refunded_amount: self.refunded_amount,
            refunded_at: self.refunded_at,
            retryable: self
                .retryable
                .ok_or_else(|| BuildError::missing_field("retryable"))?,
            settlement_currency: self
                .settlement_currency
                .ok_or_else(|| BuildError::missing_field("settlement_currency"))?,
            shipment: self.shipment,
            shipping_address: self.shipping_address,
            status: self.status,
            substatus: self
                .substatus
                .ok_or_else(|| BuildError::missing_field("substatus"))?,
            subtotal: self.subtotal,
            tax_amount: self.tax_amount,
            tax_behavior: self.tax_behavior,
            total: self.total,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            usd_total: self.usd_total,
            user: self.user,
            voidable: self
                .voidable
                .ok_or_else(|| BuildError::missing_field("voidable"))?,
        })
    }
}
