pub use crate::prelude::*;

/// The company member associated with this setup intent. Null if the user is not a member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentLegacyMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// The user for this member, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SetupIntentLegacyMemberUser>,
}

impl SetupIntentLegacyMember {
    pub fn builder() -> SetupIntentLegacyMemberBuilder {
        <SetupIntentLegacyMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentLegacyMemberBuilder {
    id: Option<String>,
    user: Option<SetupIntentLegacyMemberUser>,
}

impl SetupIntentLegacyMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user(mut self, value: SetupIntentLegacyMemberUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentLegacyMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentLegacyMemberBuilder::id)
    pub fn build(self) -> Result<SetupIntentLegacyMember, BuildError> {
        Ok(SetupIntentLegacyMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user: self.user,
        })
    }
}
