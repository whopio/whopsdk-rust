pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListShipmentsResponse {
    #[serde(default)]
    pub data: Vec<Shipment>,
    #[serde(default)]
    pub page_info: ListShipmentsResponsePageInfo,
}

impl ListShipmentsResponse {
    pub fn builder() -> ListShipmentsResponseBuilder {
        <ListShipmentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListShipmentsResponseBuilder {
    data: Option<Vec<Shipment>>,
    page_info: Option<ListShipmentsResponsePageInfo>,
}

impl ListShipmentsResponseBuilder {
    pub fn data(mut self, value: Vec<Shipment>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListShipmentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListShipmentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListShipmentsResponseBuilder::data)
    /// - [`page_info`](ListShipmentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListShipmentsResponse, BuildError> {
        Ok(ListShipmentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
