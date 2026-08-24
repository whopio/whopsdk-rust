pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogsAppsResponse {
    #[serde(default)]
    pub data: Vec<LogsAppsResponseDataItem>,
    #[serde(default)]
    pub page_info: LogsAppsResponsePageInfo,
}

impl LogsAppsResponse {
    pub fn builder() -> LogsAppsResponseBuilder {
        <LogsAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogsAppsResponseBuilder {
    data: Option<Vec<LogsAppsResponseDataItem>>,
    page_info: Option<LogsAppsResponsePageInfo>,
}

impl LogsAppsResponseBuilder {
    pub fn data(mut self, value: Vec<LogsAppsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: LogsAppsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LogsAppsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](LogsAppsResponseBuilder::data)
    /// - [`page_info`](LogsAppsResponseBuilder::page_info)
    pub fn build(self) -> Result<LogsAppsResponse, BuildError> {
        Ok(LogsAppsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
