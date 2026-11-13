pub use crate::prelude::*;

/// Who the event was about, as far as the page knew. Keys the event did not carry are left out.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostEventsCreatedPayloadDataEventsItemUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItemUser {
    pub fn builder() -> PostEventsCreatedPayloadDataEventsItemUserBuilder {
        <PostEventsCreatedPayloadDataEventsItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostEventsCreatedPayloadDataEventsItemUserBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    user_id: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItemUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostEventsCreatedPayloadDataEventsItemUser`].
    pub fn build(self) -> Result<PostEventsCreatedPayloadDataEventsItemUser, BuildError> {
        Ok(PostEventsCreatedPayloadDataEventsItemUser {
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            name: self.name,
            phone: self.phone,
            user_id: self.user_id,
        })
    }
}
