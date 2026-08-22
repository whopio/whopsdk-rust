pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppBuild {
    /// Client-generated checksum of the build file, used to verify file integrity.
    #[serde(default)]
    pub checksum: String,
    /// When the build was uploaded, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// URL to download the uploaded build artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// App build ID, prefixed `abld_`.
    #[serde(default)]
    pub id: String,
    /// Whether this build is the currently active production build for its platform.
    #[serde(default)]
    pub is_production: bool,
    /// The target platform for this build.
    pub platform: AppBuildPlatform,
    /// Feedback from the reviewer explaining a rejection, or `null` if the build has not been reviewed or was approved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_message: Option<String>,
    /// URL to download the compressed source code archive that produced this build, or `null` when the build was uploaded without a source archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// The build's review status.
    pub status: AppBuildStatus,
    #[serde(default)]
    pub supported_app_view_types: Vec<AppBuildSupportedAppViewTypesItem>,
}

impl AppBuild {
    pub fn builder() -> AppBuildBuilder {
        <AppBuildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppBuildBuilder {
    checksum: Option<String>,
    created_at: Option<String>,
    file_url: Option<String>,
    id: Option<String>,
    is_production: Option<bool>,
    platform: Option<AppBuildPlatform>,
    review_message: Option<String>,
    source_url: Option<String>,
    status: Option<AppBuildStatus>,
    supported_app_view_types: Option<Vec<AppBuildSupportedAppViewTypesItem>>,
}

impl AppBuildBuilder {
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn file_url(mut self, value: impl Into<String>) -> Self {
        self.file_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_production(mut self, value: bool) -> Self {
        self.is_production = Some(value);
        self
    }

    pub fn platform(mut self, value: AppBuildPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn review_message(mut self, value: impl Into<String>) -> Self {
        self.review_message = Some(value.into());
        self
    }

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppBuildStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn supported_app_view_types(
        mut self,
        value: Vec<AppBuildSupportedAppViewTypesItem>,
    ) -> Self {
        self.supported_app_view_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppBuild`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checksum`](AppBuildBuilder::checksum)
    /// - [`created_at`](AppBuildBuilder::created_at)
    /// - [`id`](AppBuildBuilder::id)
    /// - [`is_production`](AppBuildBuilder::is_production)
    /// - [`platform`](AppBuildBuilder::platform)
    /// - [`status`](AppBuildBuilder::status)
    /// - [`supported_app_view_types`](AppBuildBuilder::supported_app_view_types)
    pub fn build(self) -> Result<AppBuild, BuildError> {
        Ok(AppBuild {
            checksum: self
                .checksum
                .ok_or_else(|| BuildError::missing_field("checksum"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            file_url: self.file_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_production: self
                .is_production
                .ok_or_else(|| BuildError::missing_field("is_production"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            review_message: self.review_message,
            source_url: self.source_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            supported_app_view_types: self
                .supported_app_view_types
                .ok_or_else(|| BuildError::missing_field("supported_app_view_types"))?,
        })
    }
}
