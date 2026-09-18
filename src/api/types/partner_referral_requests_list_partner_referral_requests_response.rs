pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPartnerReferralRequestsResponse {
    #[serde(default)]
    pub data: Vec<PartnerReferralRequest>,
    #[serde(default)]
    pub page_info: ListPartnerReferralRequestsResponsePageInfo,
}

impl ListPartnerReferralRequestsResponse {
    pub fn builder() -> ListPartnerReferralRequestsResponseBuilder {
        <ListPartnerReferralRequestsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPartnerReferralRequestsResponseBuilder {
    data: Option<Vec<PartnerReferralRequest>>,
    page_info: Option<ListPartnerReferralRequestsResponsePageInfo>,
}

impl ListPartnerReferralRequestsResponseBuilder {
    pub fn data(mut self, value: Vec<PartnerReferralRequest>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPartnerReferralRequestsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPartnerReferralRequestsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPartnerReferralRequestsResponseBuilder::data)
    /// - [`page_info`](ListPartnerReferralRequestsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPartnerReferralRequestsResponse, BuildError> {
        Ok(ListPartnerReferralRequestsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
