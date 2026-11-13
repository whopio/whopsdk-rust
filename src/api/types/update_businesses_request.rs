pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBusinessesRequest {
    /// Annual projected card and BNPL processing combined, in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_processing_usd: Option<String>,
    /// How the partner closed this business. Send it on its own to save it; the response is then the business ID and the saved reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_closed_reason: Option<String>,
    /// Remove unaccepted projections and invalidate the offer. The original CPA term is retained if projections are later added again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_projections: Option<bool>,
}

impl UpdateBusinessesRequest {
    pub fn builder() -> UpdateBusinessesRequestBuilder {
        <UpdateBusinessesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessesRequestBuilder {
    annual_processing_usd: Option<String>,
    deal_closed_reason: Option<String>,
    withdraw_projections: Option<bool>,
}

impl UpdateBusinessesRequestBuilder {
    pub fn annual_processing_usd(mut self, value: impl Into<String>) -> Self {
        self.annual_processing_usd = Some(value.into());
        self
    }

    pub fn deal_closed_reason(mut self, value: impl Into<String>) -> Self {
        self.deal_closed_reason = Some(value.into());
        self
    }

    pub fn withdraw_projections(mut self, value: bool) -> Self {
        self.withdraw_projections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBusinessesRequest`].
    pub fn build(self) -> Result<UpdateBusinessesRequest, BuildError> {
        Ok(UpdateBusinessesRequest {
            annual_processing_usd: self.annual_processing_usd,
            deal_closed_reason: self.deal_closed_reason,
            withdraw_projections: self.withdraw_projections,
        })
    }
}
