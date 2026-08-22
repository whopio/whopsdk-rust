pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExperiencesResponse2 {
    #[serde(default)]
    pub data: Vec<ExperienceNotificationPreference>,
    #[serde(default)]
    pub page_info: ListExperiencesResponsePageInfo,
}

impl ListExperiencesResponse2 {
    pub fn builder() -> ListExperiencesResponse2Builder {
        <ListExperiencesResponse2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExperiencesResponse2Builder {
    data: Option<Vec<ExperienceNotificationPreference>>,
    page_info: Option<ListExperiencesResponsePageInfo>,
}

impl ListExperiencesResponse2Builder {
    pub fn data(mut self, value: Vec<ExperienceNotificationPreference>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListExperiencesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExperiencesResponse2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListExperiencesResponse2Builder::data)
    /// - [`page_info`](ListExperiencesResponse2Builder::page_info)
    pub fn build(self) -> Result<ListExperiencesResponse2, BuildError> {
        Ok(ListExperiencesResponse2 {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
