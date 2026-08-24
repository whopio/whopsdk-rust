pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppProductionBuild {
    /// Client-generated checksum of the build file, used to verify file integrity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// URL to download the uploaded build artifact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// App build ID, prefixed `abld_`.
    #[serde(default)]
    pub id: String,
    /// URL to download the compressed source code archive that produced this build, or `null` when the build was uploaded without a source archive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    /// The build's review status.
    pub status: AppProductionBuildStatus,
}

impl AppProductionBuild {
    pub fn builder() -> AppProductionBuildBuilder {
        <AppProductionBuildBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppProductionBuildBuilder {
    checksum: Option<String>,
    file_url: Option<String>,
    id: Option<String>,
    source_url: Option<String>,
    status: Option<AppProductionBuildStatus>,
}

impl AppProductionBuildBuilder {
    pub fn checksum(mut self, value: impl Into<String>) -> Self {
        self.checksum = Some(value.into());
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

    pub fn source_url(mut self, value: impl Into<String>) -> Self {
        self.source_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppProductionBuildStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppProductionBuild`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AppProductionBuildBuilder::id)
    /// - [`status`](AppProductionBuildBuilder::status)
    pub fn build(self) -> Result<AppProductionBuild, BuildError> {
        Ok(AppProductionBuild {
            checksum: self.checksum,
            file_url: self.file_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            source_url: self.source_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
