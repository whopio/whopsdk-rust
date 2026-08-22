pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCourseLessonsRequestAttachmentsItem {
    pub fn builder() -> UpdateCourseLessonsRequestAttachmentsItemBuilder {
        <UpdateCourseLessonsRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestAttachmentsItemBuilder {
    id: Option<String>,
}

impl UpdateCourseLessonsRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCourseLessonsRequestAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<UpdateCourseLessonsRequestAttachmentsItem, BuildError> {
        Ok(UpdateCourseLessonsRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
