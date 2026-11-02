pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentBillingDetailsPreview {
    /// ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Email supplied when the method was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Name on the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Postal or ZIP code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl PaymentBillingDetailsPreview {
    pub fn builder() -> PaymentBillingDetailsPreviewBuilder {
        <PaymentBillingDetailsPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentBillingDetailsPreviewBuilder {
    country: Option<String>,
    email: Option<String>,
    name: Option<String>,
    postal_code: Option<String>,
}

impl PaymentBillingDetailsPreviewBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    /// Consumes the builder and constructs a [`PaymentBillingDetailsPreview`].
    pub fn build(self) -> Result<PaymentBillingDetailsPreview, BuildError> {
        Ok(PaymentBillingDetailsPreview {
            country: self.country,
            email: self.email,
            name: self.name,
            postal_code: self.postal_code,
        })
    }
}
