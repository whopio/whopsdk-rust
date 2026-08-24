pub use crate::prelude::*;

/// Authorized share structure. Required when `entity_type` is `c_corp`; ignored for LLCs.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FormCompanyAccountsRequestShareStructure {
    /// Number of shares the company authorizes. Must be greater than `0`.
    #[serde(default)]
    pub number_of_shares: i64,
    /// Par value per share, in USD. Must be greater than `0`; fractional values like `0.01` are allowed.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
}

impl FormCompanyAccountsRequestShareStructure {
    pub fn builder() -> FormCompanyAccountsRequestShareStructureBuilder {
        <FormCompanyAccountsRequestShareStructureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormCompanyAccountsRequestShareStructureBuilder {
    number_of_shares: Option<i64>,
    value: Option<f64>,
}

impl FormCompanyAccountsRequestShareStructureBuilder {
    pub fn number_of_shares(mut self, value: i64) -> Self {
        self.number_of_shares = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FormCompanyAccountsRequestShareStructure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`number_of_shares`](FormCompanyAccountsRequestShareStructureBuilder::number_of_shares)
    /// - [`value`](FormCompanyAccountsRequestShareStructureBuilder::value)
    pub fn build(self) -> Result<FormCompanyAccountsRequestShareStructure, BuildError> {
        Ok(FormCompanyAccountsRequestShareStructure {
            number_of_shares: self
                .number_of_shares
                .ok_or_else(|| BuildError::missing_field("number_of_shares"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
