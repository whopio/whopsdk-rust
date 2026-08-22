pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCheckoutConfigurationsResponse {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Affiliate code applied at checkout, or `null` when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// When the checkout configuration was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Currency used for setup-mode payment method availability; defaults to `usd` when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The configuration governing a checkout mounted from this configuration, resolved through every layer (its own overrides, the plan's, and the account's) — the shape a session's `payment_method_configuration` carries. Apply it over the payment method types catalogue for the offerable set. `null` means platform defaults; `payment_method_configuration` stays this configuration's own editable override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_payment_method_configuration:
        Option<CreateCheckoutConfigurationsResponseEffectivePaymentMethodConfiguration>,
    /// Checkout configuration ID, prefixed `ch_`.
    #[serde(default)]
    pub id: String,
    /// Custom key-value metadata copied to payments and memberships. `null` without the `checkout_configuration:basic:read` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Controls whether checkout charges the buyer immediately or saves payment details for later.
    pub mode: CreateCheckoutConfigurationsResponseMode,
    /// Payment method overrides for this checkout. `null` when it uses the plan or platform defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration:
        Option<CreateCheckoutConfigurationsResponsePaymentMethodConfiguration>,
    /// Plan used for payment checkout. `null` in setup mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateCheckoutConfigurationsResponsePlan>,
    /// Checkout URL you can send to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    /// URL customers are sent to after checkout, or `null` when no redirect is configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 3D Secure behavior for this checkout, or `null` to use the account default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreateCheckoutConfigurationsResponseThreeDsLevel>,
    /// When the checkout configuration was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl CreateCheckoutConfigurationsResponse {
    pub fn builder() -> CreateCheckoutConfigurationsResponseBuilder {
        <CreateCheckoutConfigurationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsResponseBuilder {
    account_id: Option<String>,
    affiliate_code: Option<String>,
    created_at: Option<String>,
    currency: Option<String>,
    effective_payment_method_configuration:
        Option<CreateCheckoutConfigurationsResponseEffectivePaymentMethodConfiguration>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    mode: Option<CreateCheckoutConfigurationsResponseMode>,
    payment_method_configuration:
        Option<CreateCheckoutConfigurationsResponsePaymentMethodConfiguration>,
    plan: Option<CreateCheckoutConfigurationsResponsePlan>,
    purchase_url: Option<String>,
    redirect_url: Option<String>,
    three_ds_level: Option<CreateCheckoutConfigurationsResponseThreeDsLevel>,
    updated_at: Option<String>,
}

impl CreateCheckoutConfigurationsResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
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

    pub fn effective_payment_method_configuration(
        mut self,
        value: CreateCheckoutConfigurationsResponseEffectivePaymentMethodConfiguration,
    ) -> Self {
        self.effective_payment_method_configuration = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mode(mut self, value: CreateCheckoutConfigurationsResponseMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CreateCheckoutConfigurationsResponsePaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn plan(mut self, value: CreateCheckoutConfigurationsResponsePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn purchase_url(mut self, value: impl Into<String>) -> Self {
        self.purchase_url = Some(value.into());
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn three_ds_level(
        mut self,
        value: CreateCheckoutConfigurationsResponseThreeDsLevel,
    ) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateCheckoutConfigurationsResponseBuilder::account_id)
    /// - [`created_at`](CreateCheckoutConfigurationsResponseBuilder::created_at)
    /// - [`id`](CreateCheckoutConfigurationsResponseBuilder::id)
    /// - [`mode`](CreateCheckoutConfigurationsResponseBuilder::mode)
    /// - [`updated_at`](CreateCheckoutConfigurationsResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreateCheckoutConfigurationsResponse, BuildError> {
        Ok(CreateCheckoutConfigurationsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            affiliate_code: self.affiliate_code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            effective_payment_method_configuration: self.effective_payment_method_configuration,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            payment_method_configuration: self.payment_method_configuration,
            plan: self.plan,
            purchase_url: self.purchase_url,
            redirect_url: self.redirect_url,
            three_ds_level: self.three_ds_level,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
