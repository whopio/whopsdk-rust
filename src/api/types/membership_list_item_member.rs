pub use crate::prelude::*;

/// The member record linking the user to the company for this membership. Null if the member record has not been created yet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipListItemMember {
    /// The unique identifier for the member.
    #[serde(default)]
    pub id: String,
}

impl MembershipListItemMember {
    pub fn builder() -> MembershipListItemMemberBuilder {
        <MembershipListItemMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemMemberBuilder {
    id: Option<String>,
}

impl MembershipListItemMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItemMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipListItemMemberBuilder::id)
    pub fn build(self) -> Result<MembershipListItemMember, BuildError> {
        Ok(MembershipListItemMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
