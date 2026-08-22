pub use crate::prelude::*;

/// Inline billing address to create or update a mailing address for this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateInvoicesRequestBillingAddress {
    /// The city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The country of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The line 1 of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// The line 2 of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone number of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The postal code of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The type of tax identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_type: Option<TaxIdentifierTypes>,
    /// The value of the tax identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_value: Option<String>,
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
    tax_id_type: Option<TaxIdentifierTypes>,
    tax_id_value: Option<String>,
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

    pub fn tax_id_type(mut self, value: TaxIdentifierTypes) -> Self {
        self.tax_id_type = Some(value);
        self
    }

    pub fn tax_id_value(mut self, value: impl Into<String>) -> Self {
        self.tax_id_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateInvoicesRequestBillingAddress`].
    pub fn build(self) -> Result<UpdateInvoicesRequestBillingAddress, BuildError> {
        Ok(UpdateInvoicesRequestBillingAddress {
            city: self.city,
            country: self.country,
            line1: self.line1,
            line2: self.line2,
            name: self.name,
            phone: self.phone,
            postal_code: self.postal_code,
            state: self.state,
            tax_id_type: self.tax_id_type,
            tax_id_value: self.tax_id_value,
        })
    }
}
