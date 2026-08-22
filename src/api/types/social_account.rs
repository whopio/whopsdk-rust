pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SocialAccount {
    /// Why this social account currently can't be used for advertising — a failed share or a Meta-side restriction. Null when the account is healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The platform-specific ID for this social account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Unique identifier for the social account.
    #[serde(default)]
    pub id: String,
    /// The display name of the social account on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The social account this one belongs to on the platform, such as the Facebook page that owns an Instagram account. Null when the social account stands on its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_social_account: Option<SocialAccountParent>,
    /// The platform the social account exists on.
    pub platform: SocialAccountPlatform,
    /// The URL where the profile picture of the social account can be accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
    /// The URL where the social account can be accessed on the platform. Null while a Whop-owned page is still being provisioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The username of the social account on the platform. Null while a Whop-owned page is still being provisioned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether the social account is verified on the platform.
    #[serde(default)]
    pub verified: bool,
}

impl SocialAccount {
    pub fn builder() -> SocialAccountBuilder {
        <SocialAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SocialAccountBuilder {
    error: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    parent_social_account: Option<SocialAccountParent>,
    platform: Option<SocialAccountPlatform>,
    profile_picture_url: Option<String>,
    scopes: Option<Vec<String>>,
    url: Option<String>,
    username: Option<String>,
    verified: Option<bool>,
}

impl SocialAccountBuilder {
    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
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

    pub fn parent_social_account(mut self, value: SocialAccountParent) -> Self {
        self.parent_social_account = Some(value);
        self
    }

    pub fn platform(mut self, value: SocialAccountPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn profile_picture_url(mut self, value: impl Into<String>) -> Self {
        self.profile_picture_url = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`SocialAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SocialAccountBuilder::id)
    /// - [`platform`](SocialAccountBuilder::platform)
    /// - [`scopes`](SocialAccountBuilder::scopes)
    /// - [`verified`](SocialAccountBuilder::verified)
    pub fn build(self) -> Result<SocialAccount, BuildError> {
        Ok(SocialAccount {
            error: self.error,
            external_id: self.external_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            parent_social_account: self.parent_social_account,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            profile_picture_url: self.profile_picture_url,
            scopes: self
                .scopes
                .ok_or_else(|| BuildError::missing_field("scopes"))?,
            url: self.url,
            username: self.username,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
