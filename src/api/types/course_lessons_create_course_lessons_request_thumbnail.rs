pub use crate::prelude::*;

/// The thumbnail image for the lesson in PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCourseLessonsRequestThumbnail {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateCourseLessonsRequestThumbnail {
    pub fn builder() -> CreateCourseLessonsRequestThumbnailBuilder {
        <CreateCourseLessonsRequestThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCourseLessonsRequestThumbnailBuilder {
    id: Option<String>,
}

impl CreateCourseLessonsRequestThumbnailBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCourseLessonsRequestThumbnail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCourseLessonsRequestThumbnailBuilder::id)
    pub fn build(self) -> Result<CreateCourseLessonsRequestThumbnail, BuildError> {
        Ok(CreateCourseLessonsRequestThumbnail {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
