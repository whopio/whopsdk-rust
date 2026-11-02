pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentRuleFieldOption {
    /// The name to display for this value.
    #[serde(default)]
    pub label: String,
    /// The value to send in a condition.
    #[serde(default)]
    pub value: String,
}

impl PaymentRuleFieldOption {
    pub fn builder() -> PaymentRuleFieldOptionBuilder {
        <PaymentRuleFieldOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleFieldOptionBuilder {
    label: Option<String>,
    value: Option<String>,
}

impl PaymentRuleFieldOptionBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentRuleFieldOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](PaymentRuleFieldOptionBuilder::label)
    /// - [`value`](PaymentRuleFieldOptionBuilder::value)
    pub fn build(self) -> Result<PaymentRuleFieldOption, BuildError> {
        Ok(PaymentRuleFieldOption {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
