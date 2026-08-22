pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountSocialLink {
    /// The ID of the social link
    #[serde(default)]
    pub id: String,
    /// The optional display title for the social link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The social link URL
    #[serde(default)]
    pub url: String,
    /// The social platform for this link
    pub website: AccountSocialLinkWebsite,
}

impl AccountSocialLink {
    pub fn builder() -> AccountSocialLinkBuilder {
        <AccountSocialLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountSocialLinkBuilder {
    id: Option<String>,
    title: Option<String>,
    url: Option<String>,
    website: Option<AccountSocialLinkWebsite>,
}

impl AccountSocialLinkBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn website(mut self, value: AccountSocialLinkWebsite) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountSocialLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountSocialLinkBuilder::id)
    /// - [`url`](AccountSocialLinkBuilder::url)
    /// - [`website`](AccountSocialLinkBuilder::website)
    pub fn build(self) -> Result<AccountSocialLink, BuildError> {
        Ok(AccountSocialLink {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self.title,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            website: self
                .website
                .ok_or_else(|| BuildError::missing_field("website"))?,
        })
    }
}
