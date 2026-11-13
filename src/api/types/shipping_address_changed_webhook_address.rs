pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShippingAddressChangedWebhookAddress {
    /// The city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The two-letter ISO 3166-1 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state, province or region code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ShippingAddressChangedWebhookAddress {
    pub fn builder() -> ShippingAddressChangedWebhookAddressBuilder {
        <ShippingAddressChangedWebhookAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl ShippingAddressChangedWebhookAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookAddress`].
    pub fn build(self) -> Result<ShippingAddressChangedWebhookAddress, BuildError> {
        Ok(ShippingAddressChangedWebhookAddress {
            city: self.city,
            country: self.country,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
