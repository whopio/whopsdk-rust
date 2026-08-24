pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanListItem {
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
    /// When the plan was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code for this plan's prices.
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub custom_fields: Vec<PlanCustomField>,
    /// Customer-visible plan description. Maximum 1000 characters. `null` if no description is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    pub plan_type: PlanListItemPlanType,
    /// Product this plan belongs to; `null` for standalone plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<HashMap<String, serde_json::Value>>,
    /// URL where customers can purchase this plan directly.
    #[serde(default)]
    pub purchase_url: String,
    /// Sales method for this plan.
    pub release_method: PlanListItemReleaseMethod,
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
    /// 3D Secure behavior for this plan; `null` inherits the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<PlanListItemThreeDsLevel>,
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
    pub visibility: PlanListItemVisibility,
}

impl PlanListItem {
    pub fn builder() -> PlanListItemBuilder {
        <PlanListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanListItemBuilder {
    account: Option<AccountSummary>,
    adaptive_pricing_enabled: Option<bool>,
    billing_period: Option<f64>,
    cancel_discount_intervals: Option<f64>,
    cancel_discount_percentage: Option<f64>,
    checkout_styling: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<String>,
    currency: Option<String>,
    custom_fields: Option<Vec<PlanCustomField>>,
    description: Option<String>,
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
    plan_type: Option<PlanListItemPlanType>,
    product: Option<HashMap<String, serde_json::Value>>,
    purchase_url: Option<String>,
    release_method: Option<PlanListItemReleaseMethod>,
    renewal_price: Option<f64>,
    split_pay_required_payments: Option<f64>,
    stock: Option<f64>,
    strike_through_initial_price: Option<f64>,
    strike_through_renewal_price: Option<f64>,
    three_ds_level: Option<PlanListItemThreeDsLevel>,
    title: Option<String>,
    trial_period_days: Option<f64>,
    unlimited_stock: Option<bool>,
    updated_at: Option<String>,
    visibility: Option<PlanListItemVisibility>,
}

impl PlanListItemBuilder {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn custom_fields(mut self, value: Vec<PlanCustomField>) -> Self {
        self.custom_fields = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    pub fn plan_type(mut self, value: PlanListItemPlanType) -> Self {
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

    pub fn release_method(mut self, value: PlanListItemReleaseMethod) -> Self {
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

    pub fn three_ds_level(mut self, value: PlanListItemThreeDsLevel) -> Self {
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

    pub fn visibility(mut self, value: PlanListItemVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlanListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adaptive_pricing_enabled`](PlanListItemBuilder::adaptive_pricing_enabled)
    /// - [`created_at`](PlanListItemBuilder::created_at)
    /// - [`currency`](PlanListItemBuilder::currency)
    /// - [`custom_fields`](PlanListItemBuilder::custom_fields)
    /// - [`formatted_price`](PlanListItemBuilder::formatted_price)
    /// - [`id`](PlanListItemBuilder::id)
    /// - [`initial_price`](PlanListItemBuilder::initial_price)
    /// - [`plan_type`](PlanListItemBuilder::plan_type)
    /// - [`purchase_url`](PlanListItemBuilder::purchase_url)
    /// - [`release_method`](PlanListItemBuilder::release_method)
    /// - [`renewal_price`](PlanListItemBuilder::renewal_price)
    /// - [`unlimited_stock`](PlanListItemBuilder::unlimited_stock)
    /// - [`updated_at`](PlanListItemBuilder::updated_at)
    /// - [`visibility`](PlanListItemBuilder::visibility)
    pub fn build(self) -> Result<PlanListItem, BuildError> {
        Ok(PlanListItem {
            account: self.account,
            adaptive_pricing_enabled: self
                .adaptive_pricing_enabled
                .ok_or_else(|| BuildError::missing_field("adaptive_pricing_enabled"))?,
            billing_period: self.billing_period,
            cancel_discount_intervals: self.cancel_discount_intervals,
            cancel_discount_percentage: self.cancel_discount_percentage,
            checkout_styling: self.checkout_styling,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            custom_fields: self
                .custom_fields
                .ok_or_else(|| BuildError::missing_field("custom_fields"))?,
            description: self.description,
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
