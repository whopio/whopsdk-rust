pub use crate::prelude::*;

/// The company member associated with this setup intent. Null if the user is not a member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSetupIntentsResponseMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// The user for this member, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<CreateSetupIntentsResponseMemberUser>,
}

impl CreateSetupIntentsResponseMember {
    pub fn builder() -> CreateSetupIntentsResponseMemberBuilder {
        <CreateSetupIntentsResponseMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSetupIntentsResponseMemberBuilder {
    id: Option<String>,
    user: Option<CreateSetupIntentsResponseMemberUser>,
}

impl CreateSetupIntentsResponseMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user(mut self, value: CreateSetupIntentsResponseMemberUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSetupIntentsResponseMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSetupIntentsResponseMemberBuilder::id)
    pub fn build(self) -> Result<CreateSetupIntentsResponseMember, BuildError> {
        Ok(CreateSetupIntentsResponseMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user: self.user,
        })
    }
}
