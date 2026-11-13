pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportIssuesAdsResponse {
    #[serde(default)]
    pub data: Vec<AdIssue>,
}

impl ReportIssuesAdsResponse {
    pub fn builder() -> ReportIssuesAdsResponseBuilder {
        <ReportIssuesAdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportIssuesAdsResponseBuilder {
    data: Option<Vec<AdIssue>>,
}

impl ReportIssuesAdsResponseBuilder {
    pub fn data(mut self, value: Vec<AdIssue>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportIssuesAdsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ReportIssuesAdsResponseBuilder::data)
    pub fn build(self) -> Result<ReportIssuesAdsResponse, BuildError> {
        Ok(ReportIssuesAdsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
