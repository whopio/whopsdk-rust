pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLivestreamsResponse {
    #[serde(default)]
    pub data: Vec<Livestream>,
    #[serde(default)]
    pub page_info: ListLivestreamsResponsePageInfo,
}

impl ListLivestreamsResponse {
    pub fn builder() -> ListLivestreamsResponseBuilder {
        <ListLivestreamsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLivestreamsResponseBuilder {
    data: Option<Vec<Livestream>>,
    page_info: Option<ListLivestreamsResponsePageInfo>,
}

impl ListLivestreamsResponseBuilder {
    pub fn data(mut self, value: Vec<Livestream>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListLivestreamsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLivestreamsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListLivestreamsResponseBuilder::data)
    /// - [`page_info`](ListLivestreamsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListLivestreamsResponse, BuildError> {
        Ok(ListLivestreamsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
