pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubmitFinancingApplicationsRequestMerchantAcceptance {
    /// Affirmation that the merchant accepted the presented terms.
    #[serde(default)]
    pub accepted: bool,
    /// Exact `terms.version` returned by the application and presented to the merchant. A stale version is rejected.
    #[serde(default)]
    pub terms_version: String,
}

impl SubmitFinancingApplicationsRequestMerchantAcceptance {
    pub fn builder() -> SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder {
        <SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder {
    accepted: Option<bool>,
    terms_version: Option<String>,
}

impl SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder {
    pub fn accepted(mut self, value: bool) -> Self {
        self.accepted = Some(value);
        self
    }

    pub fn terms_version(mut self, value: impl Into<String>) -> Self {
        self.terms_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitFinancingApplicationsRequestMerchantAcceptance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted`](SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder::accepted)
    /// - [`terms_version`](SubmitFinancingApplicationsRequestMerchantAcceptanceBuilder::terms_version)
    pub fn build(self) -> Result<SubmitFinancingApplicationsRequestMerchantAcceptance, BuildError> {
        Ok(SubmitFinancingApplicationsRequestMerchantAcceptance {
            accepted: self
                .accepted
                .ok_or_else(|| BuildError::missing_field("accepted"))?,
            terms_version: self
                .terms_version
                .ok_or_else(|| BuildError::missing_field("terms_version"))?,
        })
    }
}
