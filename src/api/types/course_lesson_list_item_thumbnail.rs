pub use crate::prelude::*;

/// The thumbnail image displayed on lesson cards and previews. Null if no thumbnail has been uploaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonListItemThumbnail {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CourseLessonListItemThumbnail {
    pub fn builder() -> CourseLessonListItemThumbnailBuilder {
        <CourseLessonListItemThumbnailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonListItemThumbnailBuilder {
    url: Option<String>,
}

impl CourseLessonListItemThumbnailBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonListItemThumbnail`].
    pub fn build(self) -> Result<CourseLessonListItemThumbnail, BuildError> {
        Ok(CourseLessonListItemThumbnail { url: self.url })
    }
}
