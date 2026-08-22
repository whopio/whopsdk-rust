pub use crate::prelude::*;

/// The thumbnail image for the course in PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCoursesRequestThumbnail {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateCoursesRequestThumbnail {
    pub fn builder() -> CreateCoursesRequestThumbnailBuilder {
        <CreateCoursesRequestThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCoursesRequestThumbnailBuilder {
    id: Option<String>,
}

impl CreateCoursesRequestThumbnailBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCoursesRequestThumbnail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCoursesRequestThumbnailBuilder::id)
    pub fn build(self) -> Result<CreateCoursesRequestThumbnail, BuildError> {
        Ok(CreateCoursesRequestThumbnail {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
