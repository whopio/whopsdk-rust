pub use crate::prelude::*;

/// The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAppsRequestIcon {
    /// The signed id of a completed direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateAppsRequestIcon {
    pub fn builder() -> UpdateAppsRequestIconBuilder {
        <UpdateAppsRequestIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAppsRequestIconBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateAppsRequestIconBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAppsRequestIcon`].
    pub fn build(self) -> Result<UpdateAppsRequestIcon, BuildError> {
        Ok(UpdateAppsRequestIcon {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
