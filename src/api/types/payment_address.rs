pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentAddress {
    /// The city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The first street address line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// The second street address line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The name on the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state, province or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl PaymentAddress {
    pub fn builder() -> PaymentAddressBuilder {
        <PaymentAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    name: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl PaymentAddressBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`PaymentAddress`].
    pub fn build(self) -> Result<PaymentAddress, BuildError> {
        Ok(PaymentAddress {
            city: self.city,
            country: self.country,
            line1: self.line1,
            line2: self.line2,
            name: self.name,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
