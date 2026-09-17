pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePaymentRulesRequest {
    /// What this account's rule requests when every condition matches. One applicable account-rule action wins, in this order: `allow`, `block`, `review`, `enforce_3ds`. An `allow` overrides this account's other rules, never Whop's own fraud controls. A `review` requests authorization without capture for an eligible on-session card payment through Whop Payments. Automatic capture is scheduled for 48 hours after authorization; capture or void the payment before then to decide sooner. Capture may complete later or fail. Review is skipped for unsupported methods, off-session payments, and payments already configured for manual capture. An `enforce_3ds` is skipped when the account rule cannot apply a challenge. Other 3DS requirements still apply.
    pub action: ReplacePaymentRulesRequestAction,
    /// The conditions a payment is matched against. Up to 10 conditions, and 8 KiB once serialized.
    #[serde(default)]
    pub conditions: ReplacePaymentRulesRequestConditions,
}

impl ReplacePaymentRulesRequest {
    pub fn builder() -> ReplacePaymentRulesRequestBuilder {
        <ReplacePaymentRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePaymentRulesRequestBuilder {
    action: Option<ReplacePaymentRulesRequestAction>,
    conditions: Option<ReplacePaymentRulesRequestConditions>,
}

impl ReplacePaymentRulesRequestBuilder {
    pub fn action(mut self, value: ReplacePaymentRulesRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn conditions(mut self, value: ReplacePaymentRulesRequestConditions) -> Self {
        self.conditions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePaymentRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ReplacePaymentRulesRequestBuilder::action)
    /// - [`conditions`](ReplacePaymentRulesRequestBuilder::conditions)
    pub fn build(self) -> Result<ReplacePaymentRulesRequest, BuildError> {
        Ok(ReplacePaymentRulesRequest {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
        })
    }
}
