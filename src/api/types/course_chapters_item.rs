pub use crate::prelude::*;

/// A grouping of related lessons within a course, used to organize content into sections.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseChaptersItem {
    /// The unique identifier for the chapter.
    #[serde(default)]
    pub id: String,
    /// An ordered list of lessons in this chapter, sorted by display position. Hidden lessons are excluded for non-admin users.
    #[serde(default)]
    pub lessons: Vec<CourseChaptersItemLessonsItem>,
    /// The sort position of this chapter within its parent course, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The display name of the chapter shown to students. Maximum 150 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseChaptersItem {
    pub fn builder() -> CourseChaptersItemBuilder {
        <CourseChaptersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChaptersItemBuilder {
    id: Option<String>,
    lessons: Option<Vec<CourseChaptersItemLessonsItem>>,
    order: Option<i64>,
    title: Option<String>,
}

impl CourseChaptersItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lessons(mut self, value: Vec<CourseChaptersItemLessonsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`CourseChaptersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseChaptersItemBuilder::id)
    /// - [`lessons`](CourseChaptersItemBuilder::lessons)
    /// - [`order`](CourseChaptersItemBuilder::order)
    /// - [`title`](CourseChaptersItemBuilder::title)
    pub fn build(self) -> Result<CourseChaptersItem, BuildError> {
        Ok(CourseChaptersItem {
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
