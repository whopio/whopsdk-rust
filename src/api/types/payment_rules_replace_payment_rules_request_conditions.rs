pub use crate::prelude::*;

/// The conditions a payment is matched against. Up to 10 conditions, and 8 KiB once serialized.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplacePaymentRulesRequestConditions {
    #[serde(default)]
    pub all: Vec<ReplacePaymentRulesRequestConditionsAllItem>,
}

impl ReplacePaymentRulesRequestConditions {
    pub fn builder() -> ReplacePaymentRulesRequestConditionsBuilder {
        <ReplacePaymentRulesRequestConditionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePaymentRulesRequestConditionsBuilder {
    all: Option<Vec<ReplacePaymentRulesRequestConditionsAllItem>>,
}

impl ReplacePaymentRulesRequestConditionsBuilder {
    pub fn all(mut self, value: Vec<ReplacePaymentRulesRequestConditionsAllItem>) -> Self {
        self.all = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePaymentRulesRequestConditions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`all`](ReplacePaymentRulesRequestConditionsBuilder::all)
    pub fn build(self) -> Result<ReplacePaymentRulesRequestConditions, BuildError> {
        Ok(ReplacePaymentRulesRequestConditions {
            all: self.all.ok_or_else(|| BuildError::missing_field("all"))?,
        })
    }
}
