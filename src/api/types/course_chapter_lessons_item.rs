pub use crate::prelude::*;

/// An individual learning unit within a chapter, which can contain text, video, PDF, or assessment content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseChapterLessonsItem {
    /// The unique identifier for the lesson.
    #[serde(default)]
    pub id: String,
    /// The sort position of this lesson within its parent chapter, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The display name of the lesson shown to students. Maximum 120 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseChapterLessonsItem {
    pub fn builder() -> CourseChapterLessonsItemBuilder {
        <CourseChapterLessonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChapterLessonsItemBuilder {
    id: Option<String>,
    order: Option<i64>,
    title: Option<String>,
}

impl CourseChapterLessonsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`CourseChapterLessonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseChapterLessonsItemBuilder::id)
    /// - [`order`](CourseChapterLessonsItemBuilder::order)
    /// - [`title`](CourseChapterLessonsItemBuilder::title)
    pub fn build(self) -> Result<CourseChapterLessonsItem, BuildError> {
        Ok(CourseChapterLessonsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
