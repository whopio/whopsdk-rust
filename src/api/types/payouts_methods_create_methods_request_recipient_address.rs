pub use crate::prelude::*;

/// The recipient's address in recipient.country. Saved on the recipient payout account and supplied to bank methods that require address fields. Omit to retain the existing address when reusing a recipient.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMethodsRequestRecipientAddress {
    /// The recipient's city.
    #[serde(default)]
    pub city: String,
    /// The recipient's street address.
    #[serde(default)]
    pub line1: String,
    /// Apartment, suite or unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The recipient's postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The recipient's state, province or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl CreateMethodsRequestRecipientAddress {
    pub fn builder() -> CreateMethodsRequestRecipientAddressBuilder {
        <CreateMethodsRequestRecipientAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMethodsRequestRecipientAddressBuilder {
    city: Option<String>,
    line1: Option<String>,
    line2: Option<String>,
    postal_code: Option<String>,
    state: Option<String>,
}

impl CreateMethodsRequestRecipientAddressBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateMethodsRequestRecipientAddress`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](CreateMethodsRequestRecipientAddressBuilder::city)
    /// - [`line1`](CreateMethodsRequestRecipientAddressBuilder::line1)
    pub fn build(self) -> Result<CreateMethodsRequestRecipientAddress, BuildError> {
        Ok(CreateMethodsRequestRecipientAddress {
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self.line2,
            postal_code: self.postal_code,
            state: self.state,
        })
    }
}
