pub use crate::prelude::*;

/// Plan associated with the payment, when applicable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityPaymentPlan {
    /// Plan ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// Plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LedgerActivityPaymentPlan {
    pub fn builder() -> LedgerActivityPaymentPlanBuilder {
        <LedgerActivityPaymentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityPaymentPlanBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl LedgerActivityPaymentPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityPaymentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityPaymentPlanBuilder::id)
    pub fn build(self) -> Result<LedgerActivityPaymentPlan, BuildError> {
        Ok(LedgerActivityPaymentPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
