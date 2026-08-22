pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcceptResolutionCenterCasesRequestAttachmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl AcceptResolutionCenterCasesRequestAttachmentsItem {
    pub fn builder() -> AcceptResolutionCenterCasesRequestAttachmentsItemBuilder {
        <AcceptResolutionCenterCasesRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptResolutionCenterCasesRequestAttachmentsItemBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl AcceptResolutionCenterCasesRequestAttachmentsItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AcceptResolutionCenterCasesRequestAttachmentsItem`].
    pub fn build(self) -> Result<AcceptResolutionCenterCasesRequestAttachmentsItem, BuildError> {
        Ok(AcceptResolutionCenterCasesRequestAttachmentsItem {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
