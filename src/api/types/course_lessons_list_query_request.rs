pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseLessonsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the course to return all lessons across all chapters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_id: Option<String>,
    /// The unique identifier of a chapter to return only its lessons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_id: Option<String>,
}

impl CourseLessonsListQueryRequest {
    pub fn builder() -> CourseLessonsListQueryRequestBuilder {
        <CourseLessonsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseLessonsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    course_id: Option<String>,
    chapter_id: Option<String>,
}

impl CourseLessonsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn course_id(mut self, value: impl Into<String>) -> Self {
        self.course_id = Some(value.into());
        self
    }

    pub fn chapter_id(mut self, value: impl Into<String>) -> Self {
        self.chapter_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseLessonsListQueryRequest`].
    pub fn build(self) -> Result<CourseLessonsListQueryRequest, BuildError> {
        Ok(CourseLessonsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            course_id: self.course_id,
            chapter_id: self.chapter_id,
        })
    }
}
