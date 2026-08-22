pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ListEventsResponseDataItemRelatedUser {
    pub fn builder() -> ListEventsResponseDataItemRelatedUserBuilder {
        <ListEventsResponseDataItemRelatedUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedUserBuilder {
    avatar_url: Option<String>,
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl ListEventsResponseDataItemRelatedUserBuilder {
    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedUser`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedUser, BuildError> {
        Ok(ListEventsResponseDataItemRelatedUser {
            avatar_url: self.avatar_url,
            id: self.id,
            name: self.name,
            username: self.username,
        })
    }
}
