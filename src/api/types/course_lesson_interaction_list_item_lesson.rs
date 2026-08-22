pub use crate::prelude::*;

/// The lesson that this progress record belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionListItemLesson {
    /// The parent chapter that contains this lesson.
    #[serde(default)]
    pub chapter: CourseLessonInteractionListItemLessonChapter,
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseLessonInteractionListItemLesson {
    pub fn builder() -> CourseLessonInteractionListItemLessonBuilder {
        <CourseLessonInteractionListItemLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionListItemLessonBuilder {
    chapter: Option<CourseLessonInteractionListItemLessonChapter>,
    id: Option<String>,
    title: Option<String>,
}

impl CourseLessonInteractionListItemLessonBuilder {
    pub fn chapter(mut self, value: CourseLessonInteractionListItemLessonChapter) -> Self {
        self.chapter = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonInteractionListItemLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter`](CourseLessonInteractionListItemLessonBuilder::chapter)
    /// - [`id`](CourseLessonInteractionListItemLessonBuilder::id)
    /// - [`title`](CourseLessonInteractionListItemLessonBuilder::title)
    pub fn build(self) -> Result<CourseLessonInteractionListItemLesson, BuildError> {
        Ok(CourseLessonInteractionListItemLesson {
            chapter: self
                .chapter
                .ok_or_else(|| BuildError::missing_field("chapter"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
