pub use crate::prelude::*;

/// Create a billing address linked to this member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInvoicesRequestBillingAddress {
    #[serde(default)]
    pub city: String,
    /// ISO 3166-1 alpha-2 country code.
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub line1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub postal_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl UpdateInvoicesRequestBillingAddress {
    pub fn builder() -> UpdateInvoicesRequestBillingAddressBuilder {
        <UpdateInvoicesRequestBillingAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateInvoicesRequestBillingAddressBuilder {
    city: Option<String>,
    country: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl UpdateInvoicesRequestBillingAddressBuilder {
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateInvoicesRequestBillingAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](UpdateInvoicesRequestBillingAddressBuilder::city)
    /// - [`country`](UpdateInvoicesRequestBillingAddressBuilder::country)
    /// - [`line1`](UpdateInvoicesRequestBillingAddressBuilder::line1)
    /// - [`postal_code`](UpdateInvoicesRequestBillingAddressBuilder::postal_code)
    pub fn build(self) -> Result<UpdateInvoicesRequestBillingAddress, BuildError> {
        Ok(UpdateInvoicesRequestBillingAddress {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            name: self.name,
            phone: self.phone,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            state: self.state,
        })
    }
}
