pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    /// Account that sells this plan; `null` for standalone invoice plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountSummary>,
    /// Whether adaptive pricing is enabled for this plan. Raw setting — does not check processor compatibility or feature flags.
    #[serde(default)]
    pub adaptive_pricing_enabled: bool,
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_period: Option<f64>,
    /// Billing intervals the cancellation discount applies to (`0` forever, `1` first payment, or a month count). `null` when none is offered or the actor lacks the `plan:basic:read` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cancel_discount_intervals: Option<f64>,
    /// Cancellation discount as a whole-number percentage. `null` when none is offered or the actor lacks the `plan:basic:read` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cancel_discount_percentage: Option<f64>,
    /// Plan-level checkout styling (`background_color`, `button_color`, `font_family`, `border_style`); `null` inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_styling: Option<HashMap<String, serde_json::Value>>,
    /// Whether tax is collected on purchases of this plan, based on the account's tax configuration.
    #[serde(default)]
    pub collect_tax: bool,
    /// When the plan was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code for this plan's prices.
    pub currency: PlanCurrency,
    #[serde(default)]
    pub custom_fields: Vec<PlanCustomField>,
    /// Whether the plan can be deleted (it has no memberships or waitlist entries). `null` unless the actor has the `plan:basic:read` scope on the plan's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    /// Customer-visible plan description. Maximum 1000 characters. `null` if no description is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The configuration governing a checkout for this plan, resolved through every layer (the plan's own and the account's) — the shape a session's `payment_method_configuration` carries. Apply it over the payment method types catalogue for the offerable set. `null` means platform defaults; `payment_method_configuration` stays the plan's own editable override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    /// Access duration in days for expiration-based plans, such as 365 for a one-year pass. `null` for plans without an expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub expiration_days: Option<f64>,
    /// Human-readable price for display (currency + interval), e.g. "$10 / month".
    #[serde(default)]
    pub formatted_price: String,
    /// Plan ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// Pricing-tier image (`url`, `blurhash`) shown on the product page; `null` when no image is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<HashMap<String, serde_json::Value>>,
    /// Initial purchase price in plan currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub initial_price: f64,
    /// Private notes not shown to customers. `null` unless the actor has the `plan:basic:read` scope on the plan's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Invoice this plan was generated for; `null` unless created for an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<HashMap<String, serde_json::Value>>,
    /// Active memberships through this plan. `null` unless the actor has the `plan:basic:read` scope on the plan's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub member_count: Option<f64>,
    /// Custom key-value pairs stored on the plan. Included in webhook payloads for payment and membership events. Maximum 50 keys, 100 characters per key, 500 characters per value. The reserved keys `custom_cta` and `custom_cta_url`, when set, override the product's checkout call to action for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Whether a cancellation discount is offered. `null` unless the actor has the `plan:basic:read` scope on the plan's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_cancel_discount: Option<bool>,
    /// Payment method configuration (`enabled`, `disabled`, `include_platform_defaults`); `null` when plan uses default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration: Option<HashMap<String, serde_json::Value>>,
    /// Billing model for this plan.
    pub plan_type: PlanPlanType,
    /// Product this plan belongs to; `null` for standalone plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<HashMap<String, serde_json::Value>>,
    /// URL where customers can purchase this plan directly.
    #[serde(default)]
    pub purchase_url: String,
    /// Sales method for this plan.
    pub release_method: PlanReleaseMethod,
    /// Recurring price charged every billing period.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub renewal_price: f64,
    /// Installment payments required before the subscription pauses. Must be greater than 1. `null` if split pay is not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub split_pay_required_payments: Option<f64>,
    /// Units available for purchase. `null` unless the actor has the `plan:basic:read` scope on the plan's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub stock: Option<f64>,
    /// Original initial price shown with a strikethrough, in the plan's currency. `null` when no strikethrough is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub strike_through_initial_price: Option<f64>,
    /// Original renewal price shown with a strikethrough, in the plan's currency. `null` when no strikethrough is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub strike_through_renewal_price: Option<f64>,
    /// How tax is handled for this plan, including whether tax is included in the price, added at checkout, or not configured.
    pub tax_type: PlanTaxType,
    /// 3D Secure behavior for this plan; `null` inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<PlanThreeDsLevel>,
    /// Plan display name shown to customers. Maximum 30 characters. `null` if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days before the first renewal charge. `null` if no trial is configured or the user has already used a trial for this plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub trial_period_days: Option<f64>,
    /// Whether the plan has unlimited stock. When `true`, the `stock` field is ignored; waitlist plans always report `true`.
    #[serde(default)]
    pub unlimited_stock: bool,
    /// When the plan was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// Controls where this plan can be seen. When `hidden`, the plan is reachable only by its direct link.
    pub visibility: PlanVisibility,
}

impl Plan {
    pub fn builder() -> PlanBuilder {
        <PlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanBuilder {
    account: Option<AccountSummary>,
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<f64>,
    cancel_discount_intervals: Option<f64>,
    cancel_discount_percentage: Option<f64>,
    checkout_styling: Option<HashMap<String, serde_json::Value>>,
    collect_tax: Option<bool>,
    created_at: Option<String>,
    currency: Option<PlanCurrency>,
    custom_fields: Option<Vec<PlanCustomField>>,
    deletable: Option<bool>,
    description: Option<String>,
    effective_payment_method_configuration: Option<CheckoutSessionPaymentMethodConfiguration>,
    expiration_days: Option<f64>,
    formatted_price: Option<String>,
    id: Option<String>,
    image: Option<HashMap<String, serde_json::Value>>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    invoice: Option<HashMap<String, serde_json::Value>>,
    member_count: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    offer_cancel_discount: Option<bool>,
    payment_method_configuration: Option<HashMap<String, serde_json::Value>>,
    plan_type: Option<PlanPlanType>,
    product: Option<HashMap<String, serde_json::Value>>,
    purchase_url: Option<String>,
    release_method: Option<PlanReleaseMethod>,
    renewal_price: Option<f64>,
    split_pay_required_payments: Option<f64>,
    stock: Option<f64>,
    strike_through_initial_price: Option<f64>,
    strike_through_renewal_price: Option<f64>,
    tax_type: Option<PlanTaxType>,
    three_ds_level: Option<PlanThreeDsLevel>,
    title: Option<String>,
    trial_period_days: Option<f64>,
    unlimited_stock: Option<bool>,
    updated_at: Option<String>,
    visibility: Option<PlanVisibility>,
}

impl PlanBuilder {
    pub fn account(mut self, value: AccountSummary) -> Self {
        self.account = Some(value);
        self
    }

    pub fn adaptive_pricing_enabled(mut self, value: bool) -> Self {
        self.adaptive_pricing_enabled = Some(value);
        self
    }

    pub fn billing_period(mut self, value: f64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn cancel_discount_intervals(mut self, value: f64) -> Self {
        self.cancel_discount_intervals = Some(value);
        self
    }

    pub fn cancel_discount_percentage(mut self, value: f64) -> Self {
        self.cancel_discount_percentage = Some(value);
        self
    }

    pub fn checkout_styling(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.checkout_styling = Some(value);
        self
    }

    pub fn collect_tax(mut self, value: bool) -> Self {
        self.collect_tax = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: PlanCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn custom_fields(mut self, value: Vec<PlanCustomField>) -> Self {
        self.custom_fields = Some(value);
        self
    }

    pub fn deletable(mut self, value: bool) -> Self {
        self.deletable = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn effective_payment_method_configuration(
        mut self,
        value: CheckoutSessionPaymentMethodConfiguration,
    ) -> Self {
        self.effective_payment_method_configuration = Some(value);
        self
    }

    pub fn expiration_days(mut self, value: f64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn formatted_price(mut self, value: impl Into<String>) -> Self {
        self.formatted_price = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.image = Some(value);
        self
    }

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn internal_notes(mut self, value: impl Into<String>) -> Self {
        self.internal_notes = Some(value.into());
        self
    }

    pub fn invoice(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.invoice = Some(value);
        self
    }

    pub fn member_count(mut self, value: f64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn offer_cancel_discount(mut self, value: bool) -> Self {
        self.offer_cancel_discount = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: HashMap<String, serde_json::Value>,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn plan_type(mut self, value: PlanPlanType) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn product(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product = Some(value);
        self
    }

    pub fn purchase_url(mut self, value: impl Into<String>) -> Self {
        self.purchase_url = Some(value.into());
        self
    }

    pub fn release_method(mut self, value: PlanReleaseMethod) -> Self {
        self.release_method = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn split_pay_required_payments(mut self, value: f64) -> Self {
        self.split_pay_required_payments = Some(value);
        self
    }

    pub fn stock(mut self, value: f64) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn strike_through_initial_price(mut self, value: f64) -> Self {
        self.strike_through_initial_price = Some(value);
        self
    }

    pub fn strike_through_renewal_price(mut self, value: f64) -> Self {
        self.strike_through_renewal_price = Some(value);
        self
    }

    pub fn tax_type(mut self, value: PlanTaxType) -> Self {
        self.tax_type = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: PlanThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_period_days(mut self, value: f64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn visibility(mut self, value: PlanVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Plan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adaptive_pricing_enabled`](PlanBuilder::adaptive_pricing_enabled)
    /// - [`collect_tax`](PlanBuilder::collect_tax)
    /// - [`created_at`](PlanBuilder::created_at)
    /// - [`currency`](PlanBuilder::currency)
    /// - [`custom_fields`](PlanBuilder::custom_fields)
    /// - [`formatted_price`](PlanBuilder::formatted_price)
    /// - [`id`](PlanBuilder::id)
    /// - [`initial_price`](PlanBuilder::initial_price)
    /// - [`plan_type`](PlanBuilder::plan_type)
    /// - [`purchase_url`](PlanBuilder::purchase_url)
    /// - [`release_method`](PlanBuilder::release_method)
    /// - [`renewal_price`](PlanBuilder::renewal_price)
    /// - [`tax_type`](PlanBuilder::tax_type)
    /// - [`unlimited_stock`](PlanBuilder::unlimited_stock)
    /// - [`updated_at`](PlanBuilder::updated_at)
    /// - [`visibility`](PlanBuilder::visibility)
    pub fn build(self) -> Result<Plan, BuildError> {
        Ok(Plan {
            account: self.account,
            adaptive_pricing_enabled: self
                .adaptive_pricing_enabled
                .ok_or_else(|| BuildError::missing_field("adaptive_pricing_enabled"))?,
            billing_period: self.billing_period,
            cancel_discount_intervals: self.cancel_discount_intervals,
            cancel_discount_percentage: self.cancel_discount_percentage,
            checkout_styling: self.checkout_styling,
            collect_tax: self
                .collect_tax
                .ok_or_else(|| BuildError::missing_field("collect_tax"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            custom_fields: self
                .custom_fields
                .ok_or_else(|| BuildError::missing_field("custom_fields"))?,
            deletable: self.deletable,
            description: self.description,
            effective_payment_method_configuration: self.effective_payment_method_configuration,
            expiration_days: self.expiration_days,
            formatted_price: self
                .formatted_price
                .ok_or_else(|| BuildError::missing_field("formatted_price"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image: self.image,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            internal_notes: self.internal_notes,
            invoice: self.invoice,
            member_count: self.member_count,
            metadata: self.metadata,
            offer_cancel_discount: self.offer_cancel_discount,
            payment_method_configuration: self.payment_method_configuration,
            plan_type: self
                .plan_type
                .ok_or_else(|| BuildError::missing_field("plan_type"))?,
            product: self.product,
            purchase_url: self
                .purchase_url
                .ok_or_else(|| BuildError::missing_field("purchase_url"))?,
            release_method: self
                .release_method
                .ok_or_else(|| BuildError::missing_field("release_method"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            split_pay_required_payments: self.split_pay_required_payments,
            stock: self.stock,
            strike_through_initial_price: self.strike_through_initial_price,
            strike_through_renewal_price: self.strike_through_renewal_price,
            tax_type: self
                .tax_type
                .ok_or_else(|| BuildError::missing_field("tax_type"))?,
            three_ds_level: self.three_ds_level,
            title: self.title,
            trial_period_days: self.trial_period_days,
            unlimited_stock: self
                .unlimited_stock
                .ok_or_else(|| BuildError::missing_field("unlimited_stock"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
