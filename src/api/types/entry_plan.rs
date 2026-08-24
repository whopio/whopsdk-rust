pub use crate::prelude::*;

/// The waitlisted plan that this entry is a signup for.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntryPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl EntryPlan {
    pub fn builder() -> EntryPlanBuilder {
        <EntryPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryPlanBuilder {
    id: Option<String>,
}

impl EntryPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntryPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryPlanBuilder::id)
    pub fn build(self) -> Result<EntryPlan, BuildError> {
        Ok(EntryPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
