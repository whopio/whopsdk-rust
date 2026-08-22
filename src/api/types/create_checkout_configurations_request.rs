pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCheckoutConfigurationsRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Affiliate code to apply to the checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// Currency used for setup-mode payment method availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Custom key-value metadata copied to payments and memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Controls whether checkout charges the buyer immediately or saves payment details for later. Defaults to `payment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CreateCheckoutConfigurationsRequestMode>,
    /// Payment method overrides for this checkout. `null` uses the plan or platform defaults.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration:
        Option<CreateCheckoutConfigurationsRequestPaymentMethodConfiguration>,
    /// Plan attributes used to create or find a plan for this checkout configuration. Mutually exclusive with `plan_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateCheckoutConfigurationsRequestPlan>,
    /// Existing plan ID, prefixed `plan_`. Mutually exclusive with `plan`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// URL customers are sent to after checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// 3D Secure behavior for this checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreateCheckoutConfigurationsRequestThreeDsLevel>,
}

impl CreateCheckoutConfigurationsRequest {
    pub fn builder() -> CreateCheckoutConfigurationsRequestBuilder {
        <CreateCheckoutConfigurationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutConfigurationsRequestBuilder {
    account_id: Option<String>,
    affiliate_code: Option<String>,
    currency: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    mode: Option<CreateCheckoutConfigurationsRequestMode>,
    payment_method_configuration:
        Option<CreateCheckoutConfigurationsRequestPaymentMethodConfiguration>,
    plan: Option<CreateCheckoutConfigurationsRequestPlan>,
    plan_id: Option<String>,
    redirect_url: Option<String>,
    three_ds_level: Option<CreateCheckoutConfigurationsRequestThreeDsLevel>,
}

impl CreateCheckoutConfigurationsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mode(mut self, value: CreateCheckoutConfigurationsRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CreateCheckoutConfigurationsRequestPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn plan(mut self, value: CreateCheckoutConfigurationsRequestPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn three_ds_level(
        mut self,
        value: CreateCheckoutConfigurationsRequestThreeDsLevel,
    ) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutConfigurationsRequest`].
    pub fn build(self) -> Result<CreateCheckoutConfigurationsRequest, BuildError> {
        Ok(CreateCheckoutConfigurationsRequest {
            account_id: self.account_id,
            affiliate_code: self.affiliate_code,
            currency: self.currency,
            metadata: self.metadata,
            mode: self.mode,
            payment_method_configuration: self.payment_method_configuration,
            plan: self.plan,
            plan_id: self.plan_id,
            redirect_url: self.redirect_url,
            three_ds_level: self.three_ds_level,
        })
    }
}
