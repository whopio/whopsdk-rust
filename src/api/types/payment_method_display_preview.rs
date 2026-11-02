pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDisplayPreview {
    /// Lowercase card brand, such as `visa` or `mastercard`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Uniquely identifies this particular card number. Matches the `fingerprint` on any payment method saved from this token, so you can recognize a card across attempts. For a wallet, this identifies the network token rather than the underlying card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// The last four digits of the card.
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
    fingerprint: Option<String>,
    last4: Option<String>,
}

impl PaymentMethodDisplayPreviewBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
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
            fingerprint: self.fingerprint,
            last4: self.last4,
        })
    }
}
