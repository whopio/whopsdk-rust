pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EventsResolutionCenterCasesResponse {
    #[serde(default)]
    pub data: Vec<ResolutionEvent>,
    #[serde(default)]
    pub page_info: EventsResolutionCenterCasesResponsePageInfo,
}

impl EventsResolutionCenterCasesResponse {
    pub fn builder() -> EventsResolutionCenterCasesResponseBuilder {
        <EventsResolutionCenterCasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventsResolutionCenterCasesResponseBuilder {
    data: Option<Vec<ResolutionEvent>>,
    page_info: Option<EventsResolutionCenterCasesResponsePageInfo>,
}

impl EventsResolutionCenterCasesResponseBuilder {
    pub fn data(mut self, value: Vec<ResolutionEvent>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: EventsResolutionCenterCasesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EventsResolutionCenterCasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](EventsResolutionCenterCasesResponseBuilder::data)
    /// - [`page_info`](EventsResolutionCenterCasesResponseBuilder::page_info)
    pub fn build(self) -> Result<EventsResolutionCenterCasesResponse, BuildError> {
        Ok(EventsResolutionCenterCasesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
