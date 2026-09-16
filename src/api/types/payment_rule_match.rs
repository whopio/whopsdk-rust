pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentRuleMatch {
    /// What the rule asked for.
    pub action: PaymentRuleMatchAction,
    /// Payment rule ID, prefixed `prule_`.
    #[serde(default)]
    pub id: String,
    /// The rule's name when it matched. Renaming the rule afterwards does not rewrite this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PaymentRuleMatch {
    pub fn builder() -> PaymentRuleMatchBuilder {
        <PaymentRuleMatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleMatchBuilder {
    action: Option<PaymentRuleMatchAction>,
    id: Option<String>,
    name: Option<String>,
}

impl PaymentRuleMatchBuilder {
    pub fn action(mut self, value: PaymentRuleMatchAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentRuleMatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](PaymentRuleMatchBuilder::action)
    /// - [`id`](PaymentRuleMatchBuilder::id)
    pub fn build(self) -> Result<PaymentRuleMatch, BuildError> {
        Ok(PaymentRuleMatch {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
