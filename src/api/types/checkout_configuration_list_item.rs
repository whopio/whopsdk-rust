pub use crate::prelude::*;

/// A checkout configuration is a reusable configuration for a checkout, including the plan, affiliate, and custom metadata. Payments and memberships created from a checkout session inherit its metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutConfigurationListItem {
    /// The ID of the account to use for the checkout configuration
    #[serde(default)]
    pub account_id: String,
    /// The affiliate code to use for the checkout configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate_code: Option<String>,
    /// The currency to use for the configuration when in 'setup' mode. This is used to target which currency specific payment methods are available. If not provided, it will default to 'usd' when in setup mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currencies>,
    /// The unique identifier for the checkout session.
    #[serde(default)]
    pub id: String,
    /// The metadata to use for the checkout configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The mode of the checkout session.
    pub mode: CheckoutModes,
    /// The explicit payment method configuration for the session, if any. This currently only works in 'setup' mode. Use the plan's payment_method_configuration for payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration:
        Option<CheckoutConfigurationListItemPaymentMethodConfiguration>,
    /// The plan to use for the checkout configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CheckoutConfigurationListItemPlan>,
    /// A URL you can send to customers to complete a checkout. It looks like `/checkout/ch_xxxx/`
    #[serde(default)]
    pub purchase_url: String,
    /// The URL to redirect the user to after the checkout configuration is created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl CheckoutConfigurationListItem {
    pub fn builder() -> CheckoutConfigurationListItemBuilder {
        <CheckoutConfigurationListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutConfigurationListItemBuilder {
    account_id: Option<String>,
    affiliate_code: Option<String>,
    currency: Option<Currencies>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    mode: Option<CheckoutModes>,
    payment_method_configuration: Option<CheckoutConfigurationListItemPaymentMethodConfiguration>,
    plan: Option<CheckoutConfigurationListItemPlan>,
    purchase_url: Option<String>,
    redirect_url: Option<String>,
}

impl CheckoutConfigurationListItemBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn affiliate_code(mut self, value: impl Into<String>) -> Self {
        self.affiliate_code = Some(value.into());
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
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

    pub fn mode(mut self, value: CheckoutModes) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn payment_method_configuration(
        mut self,
        value: CheckoutConfigurationListItemPaymentMethodConfiguration,
    ) -> Self {
        self.payment_method_configuration = Some(value);
        self
    }

    pub fn plan(mut self, value: CheckoutConfigurationListItemPlan) -> Self {
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

    /// Consumes the builder and constructs a [`CheckoutConfigurationListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CheckoutConfigurationListItemBuilder::account_id)
    /// - [`id`](CheckoutConfigurationListItemBuilder::id)
    /// - [`mode`](CheckoutConfigurationListItemBuilder::mode)
    /// - [`purchase_url`](CheckoutConfigurationListItemBuilder::purchase_url)
    pub fn build(self) -> Result<CheckoutConfigurationListItem, BuildError> {
        Ok(CheckoutConfigurationListItem {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            affiliate_code: self.affiliate_code,
            currency: self.currency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            payment_method_configuration: self.payment_method_configuration,
            plan: self.plan,
            purchase_url: self
                .purchase_url
                .ok_or_else(|| BuildError::missing_field("purchase_url"))?,
            redirect_url: self.redirect_url,
        })
    }
}
