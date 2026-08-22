pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DenyResolutionCenterCasesRequestAttachmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl DenyResolutionCenterCasesRequestAttachmentsItem {
    pub fn builder() -> DenyResolutionCenterCasesRequestAttachmentsItemBuilder {
        <DenyResolutionCenterCasesRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DenyResolutionCenterCasesRequestAttachmentsItemBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl DenyResolutionCenterCasesRequestAttachmentsItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DenyResolutionCenterCasesRequestAttachmentsItem`].
    pub fn build(self) -> Result<DenyResolutionCenterCasesRequestAttachmentsItem, BuildError> {
        Ok(DenyResolutionCenterCasesRequestAttachmentsItem {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
