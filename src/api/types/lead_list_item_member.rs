pub use crate::prelude::*;

/// The company member record if this lead has converted into a paying customer. Null if the lead has not converted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadListItemMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl LeadListItemMember {
    pub fn builder() -> LeadListItemMemberBuilder {
        <LeadListItemMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadListItemMemberBuilder {
    id: Option<String>,
}

impl LeadListItemMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadListItemMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LeadListItemMemberBuilder::id)
    pub fn build(self) -> Result<LeadListItemMember, BuildError> {
        Ok(LeadListItemMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
