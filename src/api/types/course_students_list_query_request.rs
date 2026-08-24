pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CourseStudentsListQueryRequest {
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
    /// The unique identifier of the course to list enrolled students for.
    #[serde(default)]
    pub course_id: String,
    /// A search term to filter students by name or username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl CourseStudentsListQueryRequest {
    pub fn builder() -> CourseStudentsListQueryRequestBuilder {
        <CourseStudentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CourseStudentsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    course_id: Option<String>,
    keyword: Option<String>,
}

impl CourseStudentsListQueryRequestBuilder {
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

    pub fn keyword(mut self, value: impl Into<String>) -> Self {
        self.keyword = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CourseStudentsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`course_id`](CourseStudentsListQueryRequestBuilder::course_id)
    pub fn build(self) -> Result<CourseStudentsListQueryRequest, BuildError> {
        Ok(CourseStudentsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            course_id: self
                .course_id
                .ok_or_else(|| BuildError::missing_field("course_id"))?,
            keyword: self.keyword,
        })
    }
}
