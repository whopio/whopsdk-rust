pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentsRequest {
    /// The code an affiliate link carries, which is the affiliate's username. The affiliate is credited for this payment as on a checkout session. A code naming no one eligible to earn on the product is ignored, and the payment goes ahead. A promo code with its own affiliate takes precedence. No affiliate is credited on a variant without a product or on a purchase of several variants. At most 255 characters. Not supported with a `card_present` `payment_method`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// Minutes after authorization at which Whop captures the hold automatically unless it has been voided. Requires `capture: false`. Between 5 and 5760 (4 days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_capture_after_minutes: Option<i64>,
    /// The buyer's billing address, returned on the payment as `billing_address` and used to calculate tax when Whop calculates it for the account. Only with a `paypal` `payment_method`. Only the keys you supply are kept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CreatePaymentsRequestBillingAddress>,
    /// Whether to capture a card payment immediately. Defaults to true. Pass false to place an authorization hold that must be captured in full within five days via the capture endpoint, or automatically after `auto_capture_after_minutes`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    /// A confirmation token describing a payment method the buyer just supplied. Provide this instead of `member_id` and `payment_method_id`; the buyer is resolved from the token's billing email, or from `email`. The buyer may still have a step to complete — poll the payment's status for what to do next.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_token: Option<String>,
    /// The buyer's email, resolving or creating the user the payment belongs to. With `confirmation_token` it overrides the email the token carries, and is ignored when the token was created by a signed-in buyer; with a `card_present` `payment_method` it names a buyer the point of sale already knows, otherwise the sale belongs to a guest until a buyer is attached; with a `paypal` `payment_method` it names the buyer, and when it and `member_id` are both omitted the buyer is the PayPal account that approves the payment. Ignored with `member_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The buyer's IPv4 or IPv6 address, used to assess the charge's risk and passed to the processor. Ignored unless a saved payment method is charged with `off_session` set to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// What the buyer is purchasing. Each entry takes exactly one of `variant_id` or an inline `variant`. One entry charges that variant; several entries form a cart, which requires every variant to be compatible, belong to this account, and use the same currency. Mutually exclusive with `variant_id` and `variant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreatePaymentsRequestLineItemsItem>>,
    /// The member to charge, prefixed `mber_`. Required with `payment_method_id` unless `confirmation_token` is provided. With a `paypal` `payment_method` it names the buyer instead of `email`, and must belong to a user; omit both to take the buyer from the PayPal account that approves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Custom metadata to attach to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Whether the buyer is absent from your checkout and can't authenticate. Pass true when you charge the card without the buyer, such as a renewal on your own schedule. Pass false when the buyer is present, such as a one-click upsell right after checkout: the charge is processed as a customer-initiated card-on-file payment, and if the issuer asks the buyer to authenticate, the payment's status is `requires_action` with a `next_action` to send them to, after which they return to `return_url`. From API version 2026-10-07-1 it defaults to false when `return_url` is provided and to true otherwise; before that version it defaults to true. A saved Apple Pay, Google Pay or PayPal payment method is always charged with `off_session` true. Ignored unless `member_id` and `payment_method_id` are provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,
    /// A payment method named on this call rather than described by a confirmation token. `type` names it and the member named after it carries its details. Nothing is collected on this call: the payment is created first and the buyer or device then acts on the `next_action` the status endpoint serves. `card_present` (Tap to Pay): the reader collects against the payment with the client secret in `next_action`. `paypal`: the buyer approves on PayPal's page, reached through a `redirect` `next_action`, then returns to `return_url`; `member_id` or `email` names the buyer. Mutually exclusive with `confirmation_token` and `payment_method_id`, and with `member_id` except for `paypal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<CreatePaymentsRequestPaymentMethod>,
    /// The stored payment method to charge, prefixed `payt_`. It must belong to the member. Required unless `confirmation_token` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// A payment quote from `POST /payment_quotes`, prefixed `pq_`. The payment charges exactly the quote: its purchase (its variants and quantities, or the `variant` it priced), which this body may then omit, its promo code, and its tax, the quote's `tax_amount` rather than a figure calculated now. Omit it, or send null, to have tax calculated when the payment is charged. The quote must belong to `account_id`. A purchase field you omit or send as null takes the quote's value; the buyer's email, the addresses and the payment method are this request's own, never the quote's. Whatever you do send must describe the quoted purchase: the same variants, quantities and promo code. A quote that priced `variant` takes only the same `variant` you sent to `POST /payment_quotes` (or omit the purchase to take the quoted one), never `variant_id` or `line_items`, and a quote that priced variants by id never takes `variant`. Unless the quote located no buyer (`located_by` is null), the address the payment carries (its shipping address, else its billing address) must put the buyer where the quote priced tax, by country, state and postal code; otherwise the payment is refused with 400 before the payment method is used. A quote located by IP address or by the country the buyer chose (`located_by` is `ip_address` or `presentment_country`) is a preview and cannot be paid. A quote is consumed by one payment: a declined payment keeps it and can be retried; a new payment needs a new quote. A quote cannot be charged by a processor that doesn't take quotes, such as a seller's own connected PayPal account; such a payment is refused before the payment method is used. A quote priced in a `presentment_currency` is charged in that currency at the quote's `exchange_rate`. A payment refused over its quote carries an error `code`. `quote_expired`: the quote has expired; quote again. `quote_tax_unavailable`: the quote could not price tax; quote again, or omit `quote_id`. `quote_preview_only`: the quote was located by IP address or by the country the buyer chose; quote again with the buyer's address. `quote_in_use` (409): another payment holds the quote. `quote_mismatch`: the purchase or the buyer's address is not the one quoted. `quote_rate_expired`: the exchange rate the quote was priced at has moved too far to charge; quote again. `payment_method_currency_unsupported`: the payment method cannot pay in the quote's currency; use another method, or quote again in another currency. `quote_processor_unsupported`: the payment would be processed by a processor that doesn't take quotes, such as a seller's own connected PayPal account; use another method, or omit `quote_id` to have tax calculated when the payment is charged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    /// Where the buyer continues after completing an off-site step such as 3D Secure. An absolute https URL without credentials, at most 2,048 characters. Required when a saved payment method is charged with `off_session` false; otherwise ignored unless `confirmation_token` or a `paypal` `payment_method` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Attests that the buyer was told the payment method will be saved for later charges, and saves it once they approve, so it can be charged with `member_id` and `payment_method_id`. PayPal shows the buyer the agreement to future charges on its approval page. Omit it for a one-time payment that saves nothing. Ignored unless a `paypal` `payment_method` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentsRequestSetupFutureUsage>,
    /// Where physical goods ship, returned on the payment as `shipping_address`. Only the keys you supply are kept; omit it for digital goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreatePaymentsRequestShippingAddress>,
    /// Overrides the text on the buyer's card statement for this payment only. Takes precedence over the product's and account's custom descriptors, and changes neither. Must start with `WHOP*` unless the business processes as the merchant of record. For businesses processing as the merchant of record, omit the `WHOP*` prefix; the descriptor appears as provided. Must be 5-22 characters, contain at least one letter, and use only Latin letters, numbers, spaces, underscores, hyphens, or asterisks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// The account the purchase belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The promo code as the buyer typed it, matched within the account regardless of case and surrounding spaces, as checkout matches it. It must be valid for the variant. Send it or `promo_code_id`, not both; an empty or whitespace-only string counts as not sent. A code the account does not have, or one that is no longer active, is refused before anything is written, with the error code `promo_invalid`. A code this purchase cannot use, such as one with no uses left or one restricted to other variants, products or buyers, is refused with `promo_invalid` too, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    /// An active promo code to apply, prefixed `promo_`. It must belong to the account and be valid for the variant. Send it or `promo_code`, not both. A code this purchase cannot use is refused with the error code `promo_invalid`, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// The variant purchased, described by its attributes instead of an id: the variant with exactly these attributes is used, and one is created when none exists. Mutually exclusive with `variant_id` and `line_items`. Creating a variant requires `plan:create`; creating or updating a product requires the corresponding product permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<PaymentInputVariant>,
    /// The variant purchased, prefixed `plan_`. It must belong to the account. Mutually exclusive with `variant` and `line_items`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl CreatePaymentsRequest {
    pub fn builder() -> CreatePaymentsRequestBuilder {
        <CreatePaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestBuilder {
    affiliate_code: Option<String>,
    auto_capture_after_minutes: Option<i64>,
    billing_address: Option<CreatePaymentsRequestBillingAddress>,
    capture: Option<bool>,
    confirmation_token: Option<String>,
    email: Option<String>,
    ip_address: Option<String>,
    line_items: Option<Vec<CreatePaymentsRequestLineItemsItem>>,
    member_id: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    off_session: Option<bool>,
    payment_method: Option<CreatePaymentsRequestPaymentMethod>,
    payment_method_id: Option<String>,
    quote_id: Option<String>,
    return_url: Option<String>,
    setup_future_usage: Option<CreatePaymentsRequestSetupFutureUsage>,
    shipping_address: Option<CreatePaymentsRequestShippingAddress>,
    statement_descriptor: Option<String>,
    account_id: Option<String>,
    promo_code: Option<String>,
    promo_code_id: Option<String>,
    variant: Option<PaymentInputVariant>,
    variant_id: Option<String>,
}

impl CreatePaymentsRequestBuilder {
    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn auto_capture_after_minutes(mut self, value: i64) -> Self {
        self.auto_capture_after_minutes = Some(value);
        self
    }

    pub fn billing_address(mut self, value: CreatePaymentsRequestBillingAddress) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn capture(mut self, value: bool) -> Self {
        self.capture = Some(value);
        self
    }

    pub fn confirmation_token(mut self, value: impl Into<String>) -> Self {
        self.confirmation_token = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<CreatePaymentsRequestLineItemsItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn off_session(mut self, value: bool) -> Self {
        self.off_session = Some(value);
        self
    }

    pub fn payment_method(mut self, value: CreatePaymentsRequestPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn quote_id(mut self, value: impl Into<String>) -> Self {
        self.quote_id = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn setup_future_usage(mut self, value: CreatePaymentsRequestSetupFutureUsage) -> Self {
        self.setup_future_usage = Some(value);
        self
    }

    pub fn shipping_address(mut self, value: CreatePaymentsRequestShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.statement_descriptor = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn variant(mut self, value: PaymentInputVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePaymentsRequestBuilder::account_id)
    pub fn build(self) -> Result<CreatePaymentsRequest, BuildError> {
        Ok(CreatePaymentsRequest {
            affiliate_code: self.affiliate_code,
            auto_capture_after_minutes: self.auto_capture_after_minutes,
            billing_address: self.billing_address,
            capture: self.capture,
            confirmation_token: self.confirmation_token,
            email: self.email,
            ip_address: self.ip_address,
            line_items: self.line_items,
            member_id: self.member_id,
            metadata: self.metadata,
            off_session: self.off_session,
            payment_method: self.payment_method,
            payment_method_id: self.payment_method_id,
            quote_id: self.quote_id,
            return_url: self.return_url,
            setup_future_usage: self.setup_future_usage,
            shipping_address: self.shipping_address,
            statement_descriptor: self.statement_descriptor,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            promo_code: self.promo_code,
            promo_code_id: self.promo_code_id,
            variant: self.variant,
            variant_id: self.variant_id,
        })
    }
}
