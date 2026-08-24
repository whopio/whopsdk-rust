pub use crate::prelude::*;

/// Input for updating a lesson while updating a course
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCoursesRequestChaptersItemLessonsItem {
    /// The ID of the chapter this lesson belongs to (for moving between chapters)
    #[serde(default)]
    pub chapter_id: String,
    /// The ID of the lesson to update
    #[serde(default)]
    pub id: String,
    /// The order of the lesson within its chapter
    #[serde(default)]
    pub order: i64,
    /// The title of the lesson
    #[serde(default)]
    pub title: String,
}

impl UpdateCoursesRequestChaptersItemLessonsItem {
    pub fn builder() -> UpdateCoursesRequestChaptersItemLessonsItemBuilder {
        <UpdateCoursesRequestChaptersItemLessonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCoursesRequestChaptersItemLessonsItemBuilder {
    chapter_id: Option<String>,
    id: Option<String>,
    order: Option<i64>,
    title: Option<String>,
}

impl UpdateCoursesRequestChaptersItemLessonsItemBuilder {
    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`UpdateCoursesRequestChaptersItemLessonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chapter_id`](UpdateCoursesRequestChaptersItemLessonsItemBuilder::chapter_id)
    /// - [`id`](UpdateCoursesRequestChaptersItemLessonsItemBuilder::id)
    /// - [`order`](UpdateCoursesRequestChaptersItemLessonsItemBuilder::order)
    /// - [`title`](UpdateCoursesRequestChaptersItemLessonsItemBuilder::title)
    pub fn build(self) -> Result<UpdateCoursesRequestChaptersItemLessonsItem, BuildError> {
        Ok(UpdateCoursesRequestChaptersItemLessonsItem {
            chapter_id: self
                .chapter_id
                .ok_or_else(|| BuildError::missing_field("chapter_id"))?,
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
