pub use crate::prelude::*;

/// The thumbnail image for the lesson in PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCourseLessonsRequestThumbnail {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCourseLessonsRequestThumbnail {
    pub fn builder() -> UpdateCourseLessonsRequestThumbnailBuilder {
        <UpdateCourseLessonsRequestThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCourseLessonsRequestThumbnailBuilder {
    id: Option<String>,
}

impl UpdateCourseLessonsRequestThumbnailBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCourseLessonsRequestThumbnail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCourseLessonsRequestThumbnailBuilder::id)
    pub fn build(self) -> Result<UpdateCourseLessonsRequestThumbnail, BuildError> {
        Ok(UpdateCourseLessonsRequestThumbnail {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
