pub use crate::prelude::*;

/// A grouping of related lessons within a course, used to organize content into sections.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseChapterListItem {
    /// The unique identifier for the chapter.
    #[serde(default)]
    pub id: String,
    /// The sort position of this chapter within its parent course, starting from zero.
    #[serde(default)]
    pub order: i64,
    /// The display name of the chapter shown to students. Maximum 150 characters.
    #[serde(default)]
    pub title: String,
}

impl CourseChapterListItem {
    pub fn builder() -> CourseChapterListItemBuilder {
        <CourseChapterListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseChapterListItemBuilder {
    id: Option<String>,
    order: Option<i64>,
    title: Option<String>,
}

impl CourseChapterListItemBuilder {
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

    /// Consumes the builder and constructs a [`CourseChapterListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CourseChapterListItemBuilder::id)
    /// - [`order`](CourseChapterListItemBuilder::order)
    /// - [`title`](CourseChapterListItemBuilder::title)
    pub fn build(self) -> Result<CourseChapterListItem, BuildError> {
        Ok(CourseChapterListItem {
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
