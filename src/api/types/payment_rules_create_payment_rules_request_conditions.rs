pub use crate::prelude::*;

/// The conditions a payment is matched against. Up to 10 conditions, and 8 KiB once serialized.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentRulesRequestConditions {
    #[serde(default)]
    pub all: Vec<CreatePaymentRulesRequestConditionsAllItem>,
}

impl CreatePaymentRulesRequestConditions {
    pub fn builder() -> CreatePaymentRulesRequestConditionsBuilder {
        <CreatePaymentRulesRequestConditionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentRulesRequestConditionsBuilder {
    all: Option<Vec<CreatePaymentRulesRequestConditionsAllItem>>,
}

impl CreatePaymentRulesRequestConditionsBuilder {
    pub fn all(mut self, value: Vec<CreatePaymentRulesRequestConditionsAllItem>) -> Self {
        self.all = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentRulesRequestConditions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all`](CreatePaymentRulesRequestConditionsBuilder::all)
    pub fn build(self) -> Result<CreatePaymentRulesRequestConditions, BuildError> {
        Ok(CreatePaymentRulesRequestConditions {
            all: self.all.ok_or_else(|| BuildError::missing_field("all"))?,
        })
    }
}
