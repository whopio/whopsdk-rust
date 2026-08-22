pub use crate::prelude::*;

/// The uploaded build file: `{ id }` for an existing file or `{ direct_upload_id }` for a completed direct upload.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppBuildsRequestAttachment {
    /// The signed id of a completed direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateAppBuildsRequestAttachment {
    pub fn builder() -> CreateAppBuildsRequestAttachmentBuilder {
        <CreateAppBuildsRequestAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppBuildsRequestAttachmentBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl CreateAppBuildsRequestAttachmentBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAppBuildsRequestAttachment`].
    pub fn build(self) -> Result<CreateAppBuildsRequestAttachment, BuildError> {
        Ok(CreateAppBuildsRequestAttachment {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
