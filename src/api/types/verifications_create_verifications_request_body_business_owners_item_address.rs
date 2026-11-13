pub use crate::prelude::*;

/// The new owner's home address, with `line1`, `city`, `state`, and `postal_code`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVerificationsRequestBodyBusinessOwnersItemAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter ISO 3166-1 country code, for example `US`, `DE`, or `GB`. An address in a [sanctioned country or region](/trust-and-safety/trust-safety-overview/sanctioned-countries) is rejected with the error code `address_country_sanctioned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// First line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Second line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, province, or region code, for example `CA`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CreateVerificationsRequestBodyBusinessOwnersItemAddress {
    pub fn builder() -> CreateVerificationsRequestBodyBusinessOwnersItemAddressBuilder {
        <CreateVerificationsRequestBodyBusinessOwnersItemAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsRequestBodyBusinessOwnersItemAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CreateVerificationsRequestBodyBusinessOwnersItemAddressBuilder {
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

    /// Consumes the builder and constructs a [`CreateVerificationsRequestBodyBusinessOwnersItemAddress`].
    pub fn build(
        self,
    ) -> Result<CreateVerificationsRequestBodyBusinessOwnersItemAddress, BuildError> {
        Ok(CreateVerificationsRequestBodyBusinessOwnersItemAddress {
            city: self.city,
            country: self.country,
            line1: self.line1,
            line2: self.line2,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
