pub use crate::prelude::*;

/// The user attached to this affiliate
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffiliateListItemUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The display name set on the user's Whop profile. Null if the user has not set a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The unique username chosen by the user for their Whop profile. Null if the user has not set a username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl AffiliateListItemUser {
    pub fn builder() -> AffiliateListItemUserBuilder {
        <AffiliateListItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffiliateListItemUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl AffiliateListItemUserBuilder {
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

    /// Consumes the builder and constructs a [`AffiliateListItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AffiliateListItemUserBuilder::id)
    pub fn build(self) -> Result<AffiliateListItemUser, BuildError> {
        Ok(AffiliateListItemUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self.username,
        })
    }
}
