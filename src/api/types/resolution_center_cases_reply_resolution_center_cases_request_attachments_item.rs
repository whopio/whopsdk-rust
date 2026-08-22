pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplyResolutionCenterCasesRequestAttachmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl ReplyResolutionCenterCasesRequestAttachmentsItem {
    pub fn builder() -> ReplyResolutionCenterCasesRequestAttachmentsItemBuilder {
        <ReplyResolutionCenterCasesRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplyResolutionCenterCasesRequestAttachmentsItemBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl ReplyResolutionCenterCasesRequestAttachmentsItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplyResolutionCenterCasesRequestAttachmentsItem`].
    pub fn build(self) -> Result<ReplyResolutionCenterCasesRequestAttachmentsItem, BuildError> {
        Ok(ReplyResolutionCenterCasesRequestAttachmentsItem {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
