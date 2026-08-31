pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDisplayPreview {
    /// Lowercase card brand, e.g. `visa`. Absent when the method carries no brand.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Last four digits of the instrument. Absent when the method carries none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentMethodDisplayPreview {
    pub fn builder() -> PaymentMethodDisplayPreviewBuilder {
        <PaymentMethodDisplayPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDisplayPreviewBuilder {
    brand: Option<String>,
    last4: Option<String>,
}

impl PaymentMethodDisplayPreviewBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDisplayPreview`].
    pub fn build(self) -> Result<PaymentMethodDisplayPreview, BuildError> {
        Ok(PaymentMethodDisplayPreview {
            brand: self.brand,
            last4: self.last4,
        })
    }
}
