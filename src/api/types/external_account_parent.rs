pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExternalAccountParent {
    /// The platform-specific ID for the parent external account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// External account ID, prefixed `sacc_`.
    #[serde(default)]
    pub id: String,
    /// The display name of the parent external account on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The platform the parent external account exists on.
    pub platform: ExternalAccountParentPlatform,
    /// The URL where the profile picture of the parent external account can be accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture_url: Option<String>,
    /// The username of the parent external account on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Whether the parent external account is verified on the platform.
    #[serde(default)]
    pub verified: bool,
}

impl ExternalAccountParent {
    pub fn builder() -> ExternalAccountParentBuilder {
        <ExternalAccountParentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalAccountParentBuilder {
    external_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    platform: Option<ExternalAccountParentPlatform>,
    profile_picture_url: Option<String>,
    username: Option<String>,
    verified: Option<bool>,
}

impl ExternalAccountParentBuilder {
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

    pub fn platform(mut self, value: ExternalAccountParentPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn profile_picture_url(mut self, value: impl Into<String>) -> Self {
        self.profile_picture_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`ExternalAccountParent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExternalAccountParentBuilder::id)
    /// - [`platform`](ExternalAccountParentBuilder::platform)
    /// - [`verified`](ExternalAccountParentBuilder::verified)
    pub fn build(self) -> Result<ExternalAccountParent, BuildError> {
        Ok(ExternalAccountParent {
            external_id: self.external_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            profile_picture_url: self.profile_picture_url,
            username: self.username,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
