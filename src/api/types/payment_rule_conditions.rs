pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentRuleConditions {
    #[serde(default)]
    pub all: Vec<PaymentRuleCondition>,
}

impl PaymentRuleConditions {
    pub fn builder() -> PaymentRuleConditionsBuilder {
        <PaymentRuleConditionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleConditionsBuilder {
    all: Option<Vec<PaymentRuleCondition>>,
}

impl PaymentRuleConditionsBuilder {
    pub fn all(mut self, value: Vec<PaymentRuleCondition>) -> Self {
        self.all = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRuleConditions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all`](PaymentRuleConditionsBuilder::all)
    pub fn build(self) -> Result<PaymentRuleConditions, BuildError> {
        Ok(PaymentRuleConditions {
            all: self.all.ok_or_else(|| BuildError::missing_field("all"))?,
        })
    }
}
