pub use crate::prelude::*;

/// The buyer's billing address. Where tax is calculated when no shipping address is given, and the address a tax registration belongs to. A seller that collects tax on this purchase needs the buyer located by a `country` here, on `shipping_address`, a `presentment_country`, or an `ip_address`; without one the quote is refused with `quote_location_required`. Only the keys you supply are kept. The payment that consumes the quote must put the buyer in the same place, by country, state and postal code, through its own `shipping_address` or its confirmation token's billing address, or it is refused with `quote_mismatch`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentQuotesRequestAddress {
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
    /// The full name on the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, province, or region code, such as `CA`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CreatePaymentQuotesRequestAddress {
    pub fn builder() -> CreatePaymentQuotesRequestAddressBuilder {
        <CreatePaymentQuotesRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentQuotesRequestAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    name: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CreatePaymentQuotesRequestAddressBuilder {
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

    /// Consumes the builder and constructs a [`CreatePaymentQuotesRequestAddress`].
    pub fn build(self) -> Result<CreatePaymentQuotesRequestAddress, BuildError> {
        Ok(CreatePaymentQuotesRequestAddress {
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
