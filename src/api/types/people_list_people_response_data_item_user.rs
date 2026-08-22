pub use crate::prelude::*;

/// The person's primary whop user, when one of their identities is a whop account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemUser {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_pic_url: Option<String>,
    #[serde(default)]
    pub username: String,
}

impl ListPeopleResponseDataItemUser {
    pub fn builder() -> ListPeopleResponseDataItemUserBuilder {
        <ListPeopleResponseDataItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemUserBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_pic_url: Option<String>,
    username: Option<String>,
}

impl ListPeopleResponseDataItemUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_pic_url(mut self, value: impl Into<String>) -> Self {
        self.profile_pic_url = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListPeopleResponseDataItemUserBuilder::id)
    /// - [`username`](ListPeopleResponseDataItemUserBuilder::username)
    pub fn build(self) -> Result<ListPeopleResponseDataItemUser, BuildError> {
        Ok(ListPeopleResponseDataItemUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_pic_url: self.profile_pic_url,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
