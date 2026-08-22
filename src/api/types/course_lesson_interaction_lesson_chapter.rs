pub use crate::prelude::*;

/// The parent chapter that contains this lesson.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionLessonChapter {
    /// The unique identifier for the chapter.
    #[serde(default)]
    pub id: String,
}

impl CourseLessonInteractionLessonChapter {
    pub fn builder() -> CourseLessonInteractionLessonChapterBuilder {
        <CourseLessonInteractionLessonChapterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionLessonChapterBuilder {
    id: Option<String>,
}

impl CourseLessonInteractionLessonChapterBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionLessonChapter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseLessonInteractionLessonChapterBuilder::id)
    pub fn build(self) -> Result<CourseLessonInteractionLessonChapter, BuildError> {
        Ok(CourseLessonInteractionLessonChapter {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
