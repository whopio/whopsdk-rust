pub use crate::prelude::*;

/// The connection type for LessonInteraction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCourseLessonInteractionsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<CourseLessonInteractionListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListCourseLessonInteractionsResponse {
    pub fn builder() -> ListCourseLessonInteractionsResponseBuilder {
        <ListCourseLessonInteractionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCourseLessonInteractionsResponseBuilder {
    data: Option<Vec<CourseLessonInteractionListItem>>,
    page_info: Option<PageInfo>,
}

impl ListCourseLessonInteractionsResponseBuilder {
    pub fn data(mut self, value: Vec<CourseLessonInteractionListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCourseLessonInteractionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCourseLessonInteractionsResponseBuilder::data)
    /// - [`page_info`](ListCourseLessonInteractionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCourseLessonInteractionsResponse, BuildError> {
        Ok(ListCourseLessonInteractionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
