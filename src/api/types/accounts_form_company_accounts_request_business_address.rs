pub use crate::prelude::*;

/// Company mailing address. Required unless `use_registered_agent` is `true`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FormCompanyAccountsRequestBusinessAddress {
    #[serde(default)]
    pub city: String,
    /// Two-letter ISO 3166-1 country code, for example `US`.
    #[serde(default)]
    pub country: String,
    /// First line of the street address.
    #[serde(default)]
    pub line1: String,
    /// Second line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal or ZIP code.
    #[serde(default)]
    pub postal_code: String,
    /// State or region code, for example `CA`.
    #[serde(default)]
    pub state: String,
}

impl FormCompanyAccountsRequestBusinessAddress {
    pub fn builder() -> FormCompanyAccountsRequestBusinessAddressBuilder {
        <FormCompanyAccountsRequestBusinessAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormCompanyAccountsRequestBusinessAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl FormCompanyAccountsRequestBusinessAddressBuilder {
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

    /// Consumes the builder and constructs a [`FormCompanyAccountsRequestBusinessAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](FormCompanyAccountsRequestBusinessAddressBuilder::city)
    /// - [`country`](FormCompanyAccountsRequestBusinessAddressBuilder::country)
    /// - [`line1`](FormCompanyAccountsRequestBusinessAddressBuilder::line1)
    /// - [`postal_code`](FormCompanyAccountsRequestBusinessAddressBuilder::postal_code)
    /// - [`state`](FormCompanyAccountsRequestBusinessAddressBuilder::state)
    pub fn build(self) -> Result<FormCompanyAccountsRequestBusinessAddress, BuildError> {
        Ok(FormCompanyAccountsRequestBusinessAddress {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
