pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Export {
    /// When the export was requested, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// A short-lived link to download the finished file. `null` until `status` is `completed`, and again once the export has expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// When the file is deleted and the export moves to `expired`, as an ISO 8601 timestamp. Exports are retained for 30 days.
    #[serde(default)]
    pub expires_at: String,
    /// Export ID, prefixed `exprt_`.
    #[serde(default)]
    pub id: String,
    /// Estimated completion percentage from 0 to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i64>,
    /// The resource that was exported, e.g. `receipts`, `members`, or `payouts`.
    pub resource: ExportResource,
    /// `pending` or `processing` while the file is generated, `completed` when the download is ready, `failed` if it errored, `expired` once the file has been deleted.
    pub status: ExportStatus,
    /// When the export last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl Export {
    pub fn builder() -> ExportBuilder {
        <ExportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportBuilder {
    created_at: Option<String>,
    download_url: Option<String>,
    expires_at: Option<String>,
    id: Option<String>,
    progress_percent: Option<i64>,
    resource: Option<ExportResource>,
    status: Option<ExportStatus>,
    updated_at: Option<String>,
}

impl ExportBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn progress_percent(mut self, value: i64) -> Self {
        self.progress_percent = Some(value);
        self
    }

    pub fn resource(mut self, value: ExportResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn status(mut self, value: ExportStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Export`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ExportBuilder::created_at)
    /// - [`expires_at`](ExportBuilder::expires_at)
    /// - [`id`](ExportBuilder::id)
    /// - [`resource`](ExportBuilder::resource)
    /// - [`status`](ExportBuilder::status)
    /// - [`updated_at`](ExportBuilder::updated_at)
    pub fn build(self) -> Result<Export, BuildError> {
        Ok(Export {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            download_url: self.download_url,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            progress_percent: self.progress_percent,
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
