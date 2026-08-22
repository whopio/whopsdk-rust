pub use crate::prelude::*;

/// The lesson that this progress record belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonInteractionLesson {
    /// The parent chapter that contains this lesson.
    #[serde(default)]
    pub chapter: CourseLessonInteractionLessonChapter,
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseLessonInteractionLesson {
    pub fn builder() -> CourseLessonInteractionLessonBuilder {
        <CourseLessonInteractionLessonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonInteractionLessonBuilder {
    chapter: Option<CourseLessonInteractionLessonChapter>,
    id: Option<String>,
    title: Option<String>,
}

impl CourseLessonInteractionLessonBuilder {
    pub fn chapter(mut self, value: CourseLessonInteractionLessonChapter) -> Self {
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

    /// Consumes the builder and constructs a [`CourseLessonInteractionLesson`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter`](CourseLessonInteractionLessonBuilder::chapter)
    /// - [`id`](CourseLessonInteractionLessonBuilder::id)
    /// - [`title`](CourseLessonInteractionLessonBuilder::title)
    pub fn build(self) -> Result<CourseLessonInteractionLesson, BuildError> {
        Ok(CourseLessonInteractionLesson {
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
