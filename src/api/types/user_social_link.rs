pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserSocialLink {
    /// The link's URL.
    #[serde(default)]
    pub url: String,
    /// The user's handle on the platform, as entered or as reported by the connected account. `null` for links without a handle, such as a personal site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether Whop verified that the user owns the account.
    #[serde(default)]
    pub verified: bool,
    /// The platform the link points to. `website` is a personal site rather than a social platform.
    pub website: UserSocialLinkWebsite,
}

impl UserSocialLink {
    pub fn builder() -> UserSocialLinkBuilder {
        <UserSocialLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserSocialLinkBuilder {
    url: Option<String>,
    username: Option<String>,
    verified: Option<bool>,
    website: Option<UserSocialLinkWebsite>,
}

impl UserSocialLinkBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn website(mut self, value: UserSocialLinkWebsite) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserSocialLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](UserSocialLinkBuilder::url)
    /// - [`verified`](UserSocialLinkBuilder::verified)
    /// - [`website`](UserSocialLinkBuilder::website)
    pub fn build(self) -> Result<UserSocialLink, BuildError> {
        Ok(UserSocialLink {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            username: self.username,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
            website: self
                .website
                .ok_or_else(|| BuildError::missing_field("website"))?,
        })
    }
}
