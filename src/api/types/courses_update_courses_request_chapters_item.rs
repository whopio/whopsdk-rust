pub use crate::prelude::*;

/// Input for updating a chapter while updating a course
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCoursesRequestChaptersItem {
    /// The ID of the chapter to update
    #[serde(default)]
    pub id: String,
    /// The lessons to update within this chapter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lessons: Option<Vec<UpdateCoursesRequestChaptersItemLessonsItem>>,
    /// The order of the chapter within its course
    #[serde(default)]
    pub order: i64,
    /// The title of the chapter
    #[serde(default)]
    pub title: String,
}

impl UpdateCoursesRequestChaptersItem {
    pub fn builder() -> UpdateCoursesRequestChaptersItemBuilder {
        <UpdateCoursesRequestChaptersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCoursesRequestChaptersItemBuilder {
    id: Option<String>,
    lessons: Option<Vec<UpdateCoursesRequestChaptersItemLessonsItem>>,
    order: Option<i64>,
    title: Option<String>,
}

impl UpdateCoursesRequestChaptersItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lessons(mut self, value: Vec<UpdateCoursesRequestChaptersItemLessonsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateCoursesRequestChaptersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCoursesRequestChaptersItemBuilder::id)
    /// - [`order`](UpdateCoursesRequestChaptersItemBuilder::order)
    /// - [`title`](UpdateCoursesRequestChaptersItemBuilder::title)
    pub fn build(self) -> Result<UpdateCoursesRequestChaptersItem, BuildError> {
        Ok(UpdateCoursesRequestChaptersItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lessons: self.lessons,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
