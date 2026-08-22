pub use crate::prelude::*;

/// The connection type for CourseStudentType.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCourseStudentsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<CourseStudentListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListCourseStudentsResponse {
    pub fn builder() -> ListCourseStudentsResponseBuilder {
        <ListCourseStudentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCourseStudentsResponseBuilder {
    data: Option<Vec<CourseStudentListItem>>,
    page_info: Option<PageInfo>,
}

impl ListCourseStudentsResponseBuilder {
    pub fn data(mut self, value: Vec<CourseStudentListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCourseStudentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCourseStudentsResponseBuilder::data)
    /// - [`page_info`](ListCourseStudentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCourseStudentsResponse, BuildError> {
        Ok(ListCourseStudentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
