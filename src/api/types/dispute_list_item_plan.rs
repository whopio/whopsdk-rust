pub use crate::prelude::*;

/// The plan associated with the disputed payment. Null if the dispute is not linked to a specific plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl DisputeListItemPlan {
    pub fn builder() -> DisputeListItemPlanBuilder {
        <DisputeListItemPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPlanBuilder {
    id: Option<String>,
}

impl DisputeListItemPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeListItemPlanBuilder::id)
    pub fn build(self) -> Result<DisputeListItemPlan, BuildError> {
        Ok(DisputeListItemPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
