pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentRulesRequest {
    /// The account to create the rule on. Defaults to the account the request is acting for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// What this account's rule requests when every condition matches. One applicable account-rule action wins, in this order: `allow`, `block`, `review`, `enforce_3ds`. An `allow` overrides this account's other rules, never Whop's own fraud controls. A `review` requests authorization without capture for an eligible on-session card payment through Whop Payments. Automatic capture is scheduled for 48 hours after authorization; capture or void the payment before then to decide sooner. Capture may complete later or fail. Review is skipped for unsupported methods, off-session payments, and payments already configured for manual capture. An `enforce_3ds` is skipped when the account rule cannot apply a challenge. Other 3DS requirements still apply.
    pub action: CreatePaymentRulesRequestAction,
    /// The conditions a payment is matched against. Up to 10 conditions, and 8 KiB once serialized.
    #[serde(default)]
    pub conditions: CreatePaymentRulesRequestConditions,
    /// Custom string-to-string values for your integration. Maximum 50 keys, 40 characters per key, 500 characters per value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// A name for this rule. Up to 255 characters.
    #[serde(default)]
    pub name: String,
}

impl CreatePaymentRulesRequest {
    pub fn builder() -> CreatePaymentRulesRequestBuilder {
        <CreatePaymentRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentRulesRequestBuilder {
    account_id: Option<String>,
    action: Option<CreatePaymentRulesRequestAction>,
    conditions: Option<CreatePaymentRulesRequestConditions>,
    metadata: Option<HashMap<String, String>>,
    name: Option<String>,
}

impl CreatePaymentRulesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn action(mut self, value: CreatePaymentRulesRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn conditions(mut self, value: CreatePaymentRulesRequestConditions) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](CreatePaymentRulesRequestBuilder::action)
    /// - [`conditions`](CreatePaymentRulesRequestBuilder::conditions)
    /// - [`name`](CreatePaymentRulesRequestBuilder::name)
    pub fn build(self) -> Result<CreatePaymentRulesRequest, BuildError> {
        Ok(CreatePaymentRulesRequest {
            account_id: self.account_id,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
            metadata: self.metadata,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
