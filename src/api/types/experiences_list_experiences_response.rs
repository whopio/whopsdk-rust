pub use crate::prelude::*;

/// The connection type for PublicExperience.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExperiencesResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ExperienceListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListExperiencesResponse {
    pub fn builder() -> ListExperiencesResponseBuilder {
        <ListExperiencesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExperiencesResponseBuilder {
    data: Option<Vec<ExperienceListItem>>,
    page_info: Option<PageInfo>,
}

impl ListExperiencesResponseBuilder {
    pub fn data(mut self, value: Vec<ExperienceListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExperiencesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListExperiencesResponseBuilder::data)
    /// - [`page_info`](ListExperiencesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListExperiencesResponse, BuildError> {
        Ok(ListExperiencesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
