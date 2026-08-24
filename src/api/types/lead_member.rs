pub use crate::prelude::*;

/// The company member record if this lead has converted into a paying customer. Null if the lead has not converted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl LeadMember {
    pub fn builder() -> LeadMemberBuilder {
        <LeadMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadMemberBuilder {
    id: Option<String>,
}

impl LeadMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LeadMemberBuilder::id)
    pub fn build(self) -> Result<LeadMember, BuildError> {
        Ok(LeadMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
