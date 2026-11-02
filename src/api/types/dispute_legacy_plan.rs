pub use crate::prelude::*;

/// The plan associated with the disputed payment. Null if the dispute is not linked to a specific plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl DisputeLegacyPlan {
    pub fn builder() -> DisputeLegacyPlanBuilder {
        <DisputeLegacyPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyPlanBuilder {
    id: Option<String>,
}

impl DisputeLegacyPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeLegacyPlanBuilder::id)
    pub fn build(self) -> Result<DisputeLegacyPlan, BuildError> {
        Ok(DisputeLegacyPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
