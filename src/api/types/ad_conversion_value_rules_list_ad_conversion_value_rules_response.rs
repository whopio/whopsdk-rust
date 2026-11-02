pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAdConversionValueRulesResponse {
    #[serde(default)]
    pub data: Vec<AdConversionValueRule>,
    #[serde(default)]
    pub page_info: ListAdConversionValueRulesResponsePageInfo,
}

impl ListAdConversionValueRulesResponse {
    pub fn builder() -> ListAdConversionValueRulesResponseBuilder {
        <ListAdConversionValueRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdConversionValueRulesResponseBuilder {
    data: Option<Vec<AdConversionValueRule>>,
    page_info: Option<ListAdConversionValueRulesResponsePageInfo>,
}

impl ListAdConversionValueRulesResponseBuilder {
    pub fn data(mut self, value: Vec<AdConversionValueRule>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAdConversionValueRulesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAdConversionValueRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAdConversionValueRulesResponseBuilder::data)
    /// - [`page_info`](ListAdConversionValueRulesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAdConversionValueRulesResponse, BuildError> {
        Ok(ListAdConversionValueRulesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
