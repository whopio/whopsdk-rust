pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentVerificationChecks {
    /// The Address Verification Service (AVS) result for the billing street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    /// Whether the cardholder name matched the issuer's records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder_name: Option<String>,
    /// The Card Verification Value (CVV/CVC) result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_security_code: Option<String>,
    /// The Address Verification Service (AVS) result for the billing postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl PaymentVerificationChecks {
    pub fn builder() -> PaymentVerificationChecksBuilder {
        <PaymentVerificationChecksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentVerificationChecksBuilder {
    address_line1: Option<String>,
    card_holder_name: Option<String>,
    card_security_code: Option<String>,
    zip_code: Option<String>,
}

impl PaymentVerificationChecksBuilder {
    pub fn address_line1(mut self, value: impl Into<String>) -> Self {
        self.address_line1 = Some(value.into());
        self
    }

    pub fn card_holder_name(mut self, value: impl Into<String>) -> Self {
        self.card_holder_name = Some(value.into());
        self
    }

    pub fn card_security_code(mut self, value: impl Into<String>) -> Self {
        self.card_security_code = Some(value.into());
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentVerificationChecks`].
    pub fn build(self) -> Result<PaymentVerificationChecks, BuildError> {
        Ok(PaymentVerificationChecks {
            address_line1: self.address_line1,
            card_holder_name: self.card_holder_name,
            card_security_code: self.card_security_code,
            zip_code: self.zip_code,
        })
    }
}
