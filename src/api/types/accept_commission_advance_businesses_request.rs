pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcceptCommissionAdvanceBusinessesRequest {
    #[serde(default)]
    pub acknowledgments: Vec<AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem>,
    /// Reviewed payment amount in USD.
    #[serde(default)]
    pub expected_amount_usd: String,
    /// Signed offer token returned by the partner business response. Requires partner:commission_advance:accept permission to retrieve.
    #[serde(default)]
    pub offer_token: String,
    /// Reviewed CPA terms version.
    #[serde(default)]
    pub terms_version: String,
}

impl AcceptCommissionAdvanceBusinessesRequest {
    pub fn builder() -> AcceptCommissionAdvanceBusinessesRequestBuilder {
        <AcceptCommissionAdvanceBusinessesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptCommissionAdvanceBusinessesRequestBuilder {
    acknowledgments: Option<Vec<AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem>>,
    expected_amount_usd: Option<String>,
    offer_token: Option<String>,
    terms_version: Option<String>,
}

impl AcceptCommissionAdvanceBusinessesRequestBuilder {
    pub fn acknowledgments(
        mut self,
        value: Vec<AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem>,
    ) -> Self {
        self.acknowledgments = Some(value);
        self
    }

    pub fn expected_amount_usd(mut self, value: impl Into<String>) -> Self {
        self.expected_amount_usd = Some(value.into());
        self
    }

    pub fn offer_token(mut self, value: impl Into<String>) -> Self {
        self.offer_token = Some(value.into());
        self
    }

    pub fn terms_version(mut self, value: impl Into<String>) -> Self {
        self.terms_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AcceptCommissionAdvanceBusinessesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acknowledgments`](AcceptCommissionAdvanceBusinessesRequestBuilder::acknowledgments)
    /// - [`expected_amount_usd`](AcceptCommissionAdvanceBusinessesRequestBuilder::expected_amount_usd)
    /// - [`offer_token`](AcceptCommissionAdvanceBusinessesRequestBuilder::offer_token)
    /// - [`terms_version`](AcceptCommissionAdvanceBusinessesRequestBuilder::terms_version)
    pub fn build(self) -> Result<AcceptCommissionAdvanceBusinessesRequest, BuildError> {
        Ok(AcceptCommissionAdvanceBusinessesRequest {
            acknowledgments: self
                .acknowledgments
                .ok_or_else(|| BuildError::missing_field("acknowledgments"))?,
            expected_amount_usd: self
                .expected_amount_usd
                .ok_or_else(|| BuildError::missing_field("expected_amount_usd"))?,
            offer_token: self
                .offer_token
                .ok_or_else(|| BuildError::missing_field("offer_token"))?,
            terms_version: self
                .terms_version
                .ok_or_else(|| BuildError::missing_field("terms_version"))?,
        })
    }
}
