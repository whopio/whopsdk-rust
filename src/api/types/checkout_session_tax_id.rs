pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionTaxId {
    /// The registration's type, such as `eu_vat`.
    pub r#type: CheckoutSessionTaxIdType,
    /// The registration number, stored as the buyer entered it (whitespace removed).
    #[serde(default)]
    pub value: String,
}

impl CheckoutSessionTaxId {
    pub fn builder() -> CheckoutSessionTaxIdBuilder {
        <CheckoutSessionTaxIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionTaxIdBuilder {
    r#type: Option<CheckoutSessionTaxIdType>,
    value: Option<String>,
}

impl CheckoutSessionTaxIdBuilder {
    pub fn r#type(mut self, value: CheckoutSessionTaxIdType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionTaxId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CheckoutSessionTaxIdBuilder::r#type)
    /// - [`value`](CheckoutSessionTaxIdBuilder::value)
    pub fn build(self) -> Result<CheckoutSessionTaxId, BuildError> {
        Ok(CheckoutSessionTaxId {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
