pub use crate::prelude::*;

/// Business address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyBusinessAddressBusinessAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter ISO 3166-1 country code, for example `US`, `DE`, or `GB`.
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

impl UpdateVerificationsRequestBodyBusinessAddressBusinessAddress {
    pub fn builder() -> UpdateVerificationsRequestBodyBusinessAddressBusinessAddressBuilder {
        <UpdateVerificationsRequestBodyBusinessAddressBusinessAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyBusinessAddressBusinessAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressBusinessAddressBuilder {
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

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyBusinessAddressBusinessAddress`].
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsRequestBodyBusinessAddressBusinessAddress, BuildError> {
        Ok(
            UpdateVerificationsRequestBodyBusinessAddressBusinessAddress {
                city: self.city,
                country: self.country,
                line1: self.line1,
                line2: self.line2,
                postal_code: self.postal_code,
                state: self.state,
            },
        )
    }
}
