pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPromoCodesResponse {
    #[serde(default)]
    pub data: Vec<PromoCodeListItem>,
    #[serde(default)]
    pub page_info: ListPromoCodesResponsePageInfo,
}

impl ListPromoCodesResponse {
    pub fn builder() -> ListPromoCodesResponseBuilder {
        <ListPromoCodesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPromoCodesResponseBuilder {
    data: Option<Vec<PromoCodeListItem>>,
    page_info: Option<ListPromoCodesResponsePageInfo>,
}

impl ListPromoCodesResponseBuilder {
    pub fn data(mut self, value: Vec<PromoCodeListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPromoCodesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPromoCodesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPromoCodesResponseBuilder::data)
    /// - [`page_info`](ListPromoCodesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPromoCodesResponse, BuildError> {
        Ok(ListPromoCodesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
