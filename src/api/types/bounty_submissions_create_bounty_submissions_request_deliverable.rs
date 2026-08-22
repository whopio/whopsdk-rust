pub use crate::prelude::*;

/// The submitted work. Combine `urls`, `file_ids`, and `caption` freely; at least one link or file is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBountySubmissionsRequestDeliverable {
    /// Written context shown to reviewers alongside the work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// IDs of uploaded files attached as work, up to 10, each prefixed `file_`. Combinable with `urls` and `caption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Legacy shape selector; no longer selects anything. When present it must name an inline shape (`content_url` or `media`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateBountySubmissionsRequestDeliverableType>,
    /// Links to the posted work, up to 10. Combinable with `file_ids` and `caption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl CreateBountySubmissionsRequestDeliverable {
    pub fn builder() -> CreateBountySubmissionsRequestDeliverableBuilder {
        <CreateBountySubmissionsRequestDeliverableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBountySubmissionsRequestDeliverableBuilder {
    caption: Option<String>,
    file_ids: Option<Vec<String>>,
    r#type: Option<CreateBountySubmissionsRequestDeliverableType>,
    urls: Option<Vec<String>>,
}

impl CreateBountySubmissionsRequestDeliverableBuilder {
    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }

    pub fn file_ids(mut self, value: Vec<String>) -> Self {
        self.file_ids = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateBountySubmissionsRequestDeliverableType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn urls(mut self, value: Vec<String>) -> Self {
        self.urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBountySubmissionsRequestDeliverable`].
    pub fn build(self) -> Result<CreateBountySubmissionsRequestDeliverable, BuildError> {
        Ok(CreateBountySubmissionsRequestDeliverable {
            caption: self.caption,
            file_ids: self.file_ids,
            r#type: self.r#type,
            urls: self.urls,
        })
    }
}
