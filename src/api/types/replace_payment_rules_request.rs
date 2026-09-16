pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePaymentRulesRequest {
    /// What happens to a payment when every condition matches. An `allow` overrides this account's other rules only, never Whop's own fraud controls. An `enforce_3ds` is skipped where the payment cannot carry a challenge.
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
