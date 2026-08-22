pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTopicsResponse2 {
    #[serde(default)]
    pub data: Vec<UserNotificationPreference>,
    #[serde(default)]
    pub page_info: ListTopicsResponsePageInfo2,
}

impl ListTopicsResponse2 {
    pub fn builder() -> ListTopicsResponse2Builder {
        <ListTopicsResponse2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTopicsResponse2Builder {
    data: Option<Vec<UserNotificationPreference>>,
    page_info: Option<ListTopicsResponsePageInfo2>,
}

impl ListTopicsResponse2Builder {
    pub fn data(mut self, value: Vec<UserNotificationPreference>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListTopicsResponsePageInfo2) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTopicsResponse2`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTopicsResponse2Builder::data)
    /// - [`page_info`](ListTopicsResponse2Builder::page_info)
    pub fn build(self) -> Result<ListTopicsResponse2, BuildError> {
        Ok(ListTopicsResponse2 {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
