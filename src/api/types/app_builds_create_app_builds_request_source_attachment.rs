pub use crate::prelude::*;

/// An optional compressed archive (.zip or .gz) of the source code that produced this build, stored alongside the build so it can be downloaded later. Referenced like `attachment`, and must be a different file.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppBuildsRequestSourceAttachment {
    /// The signed id of a completed direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateAppBuildsRequestSourceAttachment {
    pub fn builder() -> CreateAppBuildsRequestSourceAttachmentBuilder {
        <CreateAppBuildsRequestSourceAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppBuildsRequestSourceAttachmentBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl CreateAppBuildsRequestSourceAttachmentBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAppBuildsRequestSourceAttachment`].
    pub fn build(self) -> Result<CreateAppBuildsRequestSourceAttachment, BuildError> {
        Ok(CreateAppBuildsRequestSourceAttachment {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
