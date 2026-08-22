pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalculateTaxPlansRequest {
    /// Buyer billing address used for tax calculation. Provide either `address.country` or `ip_address`; include state and postal code when available for more accurate results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CalculateTaxPlansRequestAddress>,
    /// Buyer IP address used to infer location when no billing address is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Optional buyer tax ID for B2B exemptions. At most one entry is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<CalculateTaxPlansRequestTaxIdsItem>>,
}

impl CalculateTaxPlansRequest {
    pub fn builder() -> CalculateTaxPlansRequestBuilder {
        <CalculateTaxPlansRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxPlansRequestBuilder {
    address: Option<CalculateTaxPlansRequestAddress>,
    ip_address: Option<String>,
    tax_ids: Option<Vec<CalculateTaxPlansRequestTaxIdsItem>>,
}

impl CalculateTaxPlansRequestBuilder {
    pub fn address(mut self, value: CalculateTaxPlansRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn tax_ids(mut self, value: Vec<CalculateTaxPlansRequestTaxIdsItem>) -> Self {
        self.tax_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxPlansRequest`].
    pub fn build(self) -> Result<CalculateTaxPlansRequest, BuildError> {
        Ok(CalculateTaxPlansRequest {
            address: self.address,
            ip_address: self.ip_address,
            tax_ids: self.tax_ids,
        })
    }
}
