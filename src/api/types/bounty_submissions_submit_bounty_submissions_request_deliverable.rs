pub use crate::prelude::*;

/// Work to attach to the submission. Combine `urls`, `file_ids`, and `caption` freely; all are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitBountySubmissionsRequestDeliverable {
    /// Written context shown to reviewers alongside the work.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// IDs of uploaded files attached as work, up to 10, each prefixed `file_`. Combinable with `urls` and `caption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Links to the posted work, up to 10. Combinable with `file_ids` and `caption`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

impl SubmitBountySubmissionsRequestDeliverable {
    pub fn builder() -> SubmitBountySubmissionsRequestDeliverableBuilder {
        <SubmitBountySubmissionsRequestDeliverableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitBountySubmissionsRequestDeliverableBuilder {
    caption: Option<String>,
    file_ids: Option<Vec<String>>,
    urls: Option<Vec<String>>,
}

impl SubmitBountySubmissionsRequestDeliverableBuilder {
    pub fn caption(mut self, value: impl Into<String>) -> Self {
        self.caption = Some(value.into());
        self
    }

    pub fn file_ids(mut self, value: Vec<String>) -> Self {
        self.file_ids = Some(value);
        self
    }

    pub fn urls(mut self, value: Vec<String>) -> Self {
        self.urls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitBountySubmissionsRequestDeliverable`].
    pub fn build(self) -> Result<SubmitBountySubmissionsRequestDeliverable, BuildError> {
        Ok(SubmitBountySubmissionsRequestDeliverable {
            caption: self.caption,
            file_ids: self.file_ids,
            urls: self.urls,
        })
    }
}
