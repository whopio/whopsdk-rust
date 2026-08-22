pub use crate::prelude::*;

/// A grouping of related lessons within a course, used to organize content into sections.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseChapter {
    /// The unique identifier for the chapter.
    #[serde(default)]
    pub id: String,
    /// An ordered list of lessons in this chapter, sorted by display position. Hidden lessons are excluded for non-admin users.
    #[serde(default)]
    pub lessons: Vec<CourseChapterLessonsItem>,
    /// The sort position of this chapter within its parent course, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The display name of the chapter shown to students. Maximum 150 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseChapter {
    pub fn builder() -> CourseChapterBuilder {
        <CourseChapterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChapterBuilder {
    id: Option<String>,
    lessons: Option<Vec<CourseChapterLessonsItem>>,
    order: Option<i64>,
    title: Option<String>,
}

impl CourseChapterBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lessons(mut self, value: Vec<CourseChapterLessonsItem>) -> Self {
        self.lessons = Some(value);
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseChapter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseChapterBuilder::id)
    /// - [`lessons`](CourseChapterBuilder::lessons)
    /// - [`order`](CourseChapterBuilder::order)
    /// - [`title`](CourseChapterBuilder::title)
    pub fn build(self) -> Result<CourseChapter, BuildError> {
        Ok(CourseChapter {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lessons: self
                .lessons
                .ok_or_else(|| BuildError::missing_field("lessons"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
