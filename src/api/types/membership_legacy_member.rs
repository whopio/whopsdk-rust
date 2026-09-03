pub use crate::prelude::*;

/// The member record linking the user to the company for this membership. Null if the member record has not been created yet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipLegacyMember {
    /// The unique identifier for the member.
    #[serde(default)]
    pub id: String,
}

impl MembershipLegacyMember {
    pub fn builder() -> MembershipLegacyMemberBuilder {
        <MembershipLegacyMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyMemberBuilder {
    id: Option<String>,
}

impl MembershipLegacyMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipLegacyMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipLegacyMemberBuilder::id)
    pub fn build(self) -> Result<MembershipLegacyMember, BuildError> {
        Ok(MembershipLegacyMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
