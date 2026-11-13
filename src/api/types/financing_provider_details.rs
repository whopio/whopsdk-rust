pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FinancingProviderDetails {
    /// The account's dispute rate as a fraction, for example 0.012 for 1.2%.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dispute_rate: Option<f64>,
    /// The dispute rate, as a fraction, at which the provider is turned off or held back.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dispute_rate_limit: Option<f64>,
    /// The provider whose dispute rate caused this change, or `null` when it was this provider's own rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_provider: Option<FinancingProviderDetailsSourceProvider>,
}

impl FinancingProviderDetails {
    pub fn builder() -> FinancingProviderDetailsBuilder {
        <FinancingProviderDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingProviderDetailsBuilder {
    dispute_rate: Option<f64>,
    dispute_rate_limit: Option<f64>,
    source_provider: Option<FinancingProviderDetailsSourceProvider>,
}

impl FinancingProviderDetailsBuilder {
    pub fn dispute_rate(mut self, value: f64) -> Self {
        self.dispute_rate = Some(value);
        self
    }

    pub fn dispute_rate_limit(mut self, value: f64) -> Self {
        self.dispute_rate_limit = Some(value);
        self
    }

    pub fn source_provider(mut self, value: FinancingProviderDetailsSourceProvider) -> Self {
        self.source_provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancingProviderDetails`].
    pub fn build(self) -> Result<FinancingProviderDetails, BuildError> {
        Ok(FinancingProviderDetails {
            dispute_rate: self.dispute_rate,
            dispute_rate_limit: self.dispute_rate_limit,
            source_provider: self.source_provider,
        })
    }
}
