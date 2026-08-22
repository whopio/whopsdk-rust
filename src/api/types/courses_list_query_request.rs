pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CoursesListQueryRequest {
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
    /// The unique identifier of the experience to list courses for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// The unique identifier of the company to list courses for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
}

impl CoursesListQueryRequest {
    pub fn builder() -> CoursesListQueryRequestBuilder {
        <CoursesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CoursesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    experience_id: Option<String>,
    company_id: Option<String>,
}

impl CoursesListQueryRequestBuilder {
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

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CoursesListQueryRequest`].
    pub fn build(self) -> Result<CoursesListQueryRequest, BuildError> {
        Ok(CoursesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            experience_id: self.experience_id,
            company_id: self.company_id,
        })
    }
}
