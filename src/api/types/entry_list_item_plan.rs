pub use crate::prelude::*;

/// The waitlisted plan that this entry is a signup for.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntryListItemPlan {
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl EntryListItemPlan {
    pub fn builder() -> EntryListItemPlanBuilder {
        <EntryListItemPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryListItemPlanBuilder {
    id: Option<String>,
}

impl EntryListItemPlanBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntryListItemPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryListItemPlanBuilder::id)
    pub fn build(self) -> Result<EntryListItemPlan, BuildError> {
        Ok(EntryListItemPlan {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
