pub use crate::prelude::*;

/// The user for this member, if any.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemberListItemUser {
    /// The digital mailing address of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The unique identifier for the company member user.
    #[serde(default)]
    pub id: String,
    /// The user's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The whop username.
    #[serde(default)]
    pub username: String,
}

impl MemberListItemUser {
    pub fn builder() -> MemberListItemUserBuilder {
        <MemberListItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberListItemUserBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl MemberListItemUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberListItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MemberListItemUserBuilder::id)
    /// - [`username`](MemberListItemUserBuilder::username)
    pub fn build(self) -> Result<MemberListItemUser, BuildError> {
        Ok(MemberListItemUser {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
