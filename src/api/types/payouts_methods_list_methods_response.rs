pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponse {
    #[serde(default)]
    pub data: Vec<ListMethodsResponseDataItem>,
    /// The live per-speed payout caps for the account in the requested currency — the numbers a payout request is validated against at submit time, so clients can cap an amount input at a value the request will accept. Only present when include_limits is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<ListMethodsResponseLimits>,
    #[serde(default)]
    pub page_info: ListMethodsResponsePageInfo,
}

impl ListMethodsResponse {
    pub fn builder() -> ListMethodsResponseBuilder {
        <ListMethodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseBuilder {
    data: Option<Vec<ListMethodsResponseDataItem>>,
    limits: Option<ListMethodsResponseLimits>,
    page_info: Option<ListMethodsResponsePageInfo>,
}

impl ListMethodsResponseBuilder {
    pub fn data(mut self, value: Vec<ListMethodsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn limits(mut self, value: ListMethodsResponseLimits) -> Self {
        self.limits = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListMethodsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMethodsResponseBuilder::data)
    /// - [`page_info`](ListMethodsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMethodsResponse, BuildError> {
        Ok(ListMethodsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            limits: self.limits,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
