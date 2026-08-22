pub use crate::prelude::*;

/// Buyer billing address used for tax calculation. Provide either `address.country` or `ip_address`; include state and postal code when available for more accurate results.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CalculateTaxPlansRequestAddress {
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO 3166-1 alpha-2 country code, such as `US`, `DE`, or `GB`.
    #[serde(default)]
    pub country: String,
    /// First line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Second line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, province, or region code, such as `CA`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CalculateTaxPlansRequestAddress {
    pub fn builder() -> CalculateTaxPlansRequestAddressBuilder {
        <CalculateTaxPlansRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxPlansRequestAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CalculateTaxPlansRequestAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn line1(mut self, value: impl Into<String>) -> Self {
        self.line1 = Some(value.into());
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxPlansRequestAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country`](CalculateTaxPlansRequestAddressBuilder::country)
    pub fn build(self) -> Result<CalculateTaxPlansRequestAddress, BuildError> {
        Ok(CalculateTaxPlansRequestAddress {
            city: self.city,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            line1: self.line1,
            line2: self.line2,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
