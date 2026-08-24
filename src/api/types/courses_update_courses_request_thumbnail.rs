pub use crate::prelude::*;

/// The thumbnail image for the course in PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCoursesRequestThumbnail {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCoursesRequestThumbnail {
    pub fn builder() -> UpdateCoursesRequestThumbnailBuilder {
        <UpdateCoursesRequestThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCoursesRequestThumbnailBuilder {
    id: Option<String>,
}

impl UpdateCoursesRequestThumbnailBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCoursesRequestThumbnail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCoursesRequestThumbnailBuilder::id)
    pub fn build(self) -> Result<UpdateCoursesRequestThumbnail, BuildError> {
        Ok(UpdateCoursesRequestThumbnail {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
