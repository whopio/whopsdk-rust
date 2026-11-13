pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitFinancingApplicationsRequest {
    #[serde(default)]
    pub merchant_acceptance: SubmitFinancingApplicationsRequestMerchantAcceptance,
}

impl SubmitFinancingApplicationsRequest {
    pub fn builder() -> SubmitFinancingApplicationsRequestBuilder {
        <SubmitFinancingApplicationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitFinancingApplicationsRequestBuilder {
    merchant_acceptance: Option<SubmitFinancingApplicationsRequestMerchantAcceptance>,
}

impl SubmitFinancingApplicationsRequestBuilder {
    pub fn merchant_acceptance(
        mut self,
        value: SubmitFinancingApplicationsRequestMerchantAcceptance,
    ) -> Self {
        self.merchant_acceptance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitFinancingApplicationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`merchant_acceptance`](SubmitFinancingApplicationsRequestBuilder::merchant_acceptance)
    pub fn build(self) -> Result<SubmitFinancingApplicationsRequest, BuildError> {
        Ok(SubmitFinancingApplicationsRequest {
            merchant_acceptance: self
                .merchant_acceptance
                .ok_or_else(|| BuildError::missing_field("merchant_acceptance"))?,
        })
    }
}
