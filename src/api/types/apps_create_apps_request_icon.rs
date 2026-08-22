pub use crate::prelude::*;

/// The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppsRequestIcon {
    /// The signed id of a completed direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateAppsRequestIcon {
    pub fn builder() -> CreateAppsRequestIconBuilder {
        <CreateAppsRequestIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppsRequestIconBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl CreateAppsRequestIconBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAppsRequestIcon`].
    pub fn build(self) -> Result<CreateAppsRequestIcon, BuildError> {
        Ok(CreateAppsRequestIcon {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
