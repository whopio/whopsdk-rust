pub use crate::prelude::*;

/// The connection type for Course.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCoursesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<CourseListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListCoursesResponse {
    pub fn builder() -> ListCoursesResponseBuilder {
        <ListCoursesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCoursesResponseBuilder {
    data: Option<Vec<CourseListItem>>,
    page_info: Option<PageInfo>,
}

impl ListCoursesResponseBuilder {
    pub fn data(mut self, value: Vec<CourseListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCoursesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCoursesResponseBuilder::data)
    /// - [`page_info`](ListCoursesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCoursesResponse, BuildError> {
        Ok(ListCoursesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
