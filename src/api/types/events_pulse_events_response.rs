pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PulseEventsResponse {
    /// Recent anonymized money-movement events, newest first.
    #[serde(default)]
    pub data: Vec<PulseEventsResponseDataItem>,
    #[serde(default)]
    pub page_info: PulseEventsResponsePageInfo,
}

impl PulseEventsResponse {
    pub fn builder() -> PulseEventsResponseBuilder {
        <PulseEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PulseEventsResponseBuilder {
    data: Option<Vec<PulseEventsResponseDataItem>>,
    page_info: Option<PulseEventsResponsePageInfo>,
}

impl PulseEventsResponseBuilder {
    pub fn data(mut self, value: Vec<PulseEventsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PulseEventsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PulseEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](PulseEventsResponseBuilder::data)
    /// - [`page_info`](PulseEventsResponseBuilder::page_info)
    pub fn build(self) -> Result<PulseEventsResponse, BuildError> {
        Ok(PulseEventsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
