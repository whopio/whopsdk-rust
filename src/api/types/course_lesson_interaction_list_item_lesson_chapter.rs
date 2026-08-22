pub use crate::prelude::*;

/// The parent chapter that contains this lesson.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionListItemLessonChapter {
    /// The unique identifier for the chapter.
    #[serde(default)]
    pub id: String,
}

impl CourseLessonInteractionListItemLessonChapter {
    pub fn builder() -> CourseLessonInteractionListItemLessonChapterBuilder {
        <CourseLessonInteractionListItemLessonChapterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionListItemLessonChapterBuilder {
    id: Option<String>,
}

impl CourseLessonInteractionListItemLessonChapterBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionListItemLessonChapter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseLessonInteractionListItemLessonChapterBuilder::id)
    pub fn build(self) -> Result<CourseLessonInteractionListItemLessonChapter, BuildError> {
        Ok(CourseLessonInteractionListItemLessonChapter {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
