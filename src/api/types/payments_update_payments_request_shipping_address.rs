pub use crate::prelude::*;

/// The complete new shipping address. It replaces the current address as a whole and is never merged with it, so send every field the address should have, including the ones that are not changing. Any field you leave out is cleared: sending only `city` leaves an address with nothing but a city. Pass null to remove the address, or omit `shipping_address` to leave it unchanged. It cannot change once a shipment exists for the payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentsRequestShippingAddress {
    /// City name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// ISO 3166-1 alpha-2 country code, such as `US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// First line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Second line of the street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The recipient's full name, as it should appear on the shipping label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, province, or region code, such as `CA`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl UpdatePaymentsRequestShippingAddress {
    pub fn builder() -> UpdatePaymentsRequestShippingAddressBuilder {
        <UpdatePaymentsRequestShippingAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentsRequestShippingAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    name: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl UpdatePaymentsRequestShippingAddressBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePaymentsRequestShippingAddress`].
    pub fn build(self) -> Result<UpdatePaymentsRequestShippingAddress, BuildError> {
        Ok(UpdatePaymentsRequestShippingAddress {
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
