pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListNotificationsResponse {
    #[serde(default)]
    pub data: Vec<Notification>,
    #[serde(default)]
    pub page_info: ListNotificationsResponsePageInfo,
}

impl ListNotificationsResponse {
    pub fn builder() -> ListNotificationsResponseBuilder {
        <ListNotificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListNotificationsResponseBuilder {
    data: Option<Vec<Notification>>,
    page_info: Option<ListNotificationsResponsePageInfo>,
}

impl ListNotificationsResponseBuilder {
    pub fn data(mut self, value: Vec<Notification>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListNotificationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListNotificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListNotificationsResponseBuilder::data)
    /// - [`page_info`](ListNotificationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListNotificationsResponse, BuildError> {
        Ok(ListNotificationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
