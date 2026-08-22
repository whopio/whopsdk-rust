pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentVoucher {
    /// Exactly what the buyer must pay, in the charged currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// The barcode's contents, when the voucher carries one — render it in the symbology named by `barcode_format`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode: Option<String>,
    /// The symbology `barcode` is encoded in, such as `CODE_128`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barcode_format: Option<String>,
    /// Who the payment is made out to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// A hosted page with the complete, printable instructions. If you would rather not render the details yourself, send the buyer here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    /// When the voucher stops being payable, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// URL of that network's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_logo: Option<String>,
    /// The network the buyer pays at, such as OXXO.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    /// The voucher's number — what the buyer reads out or types at the counter to pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl PaymentVoucher {
    pub fn builder() -> PaymentVoucherBuilder {
        <PaymentVoucherBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentVoucherBuilder {
    amount: Option<Money>,
    barcode: Option<String>,
    barcode_format: Option<String>,
    company_name: Option<String>,
    document_url: Option<String>,
    expires_at: Option<String>,
    provider_logo: Option<String>,
    provider_name: Option<String>,
    reference: Option<String>,
}

impl PaymentVoucherBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn barcode(mut self, value: impl Into<String>) -> Self {
        self.barcode = Some(value.into());
        self
    }

    pub fn barcode_format(mut self, value: impl Into<String>) -> Self {
        self.barcode_format = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn document_url(mut self, value: impl Into<String>) -> Self {
        self.document_url = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn provider_logo(mut self, value: impl Into<String>) -> Self {
        self.provider_logo = Some(value.into());
        self
    }

    pub fn provider_name(mut self, value: impl Into<String>) -> Self {
        self.provider_name = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentVoucher`].
    pub fn build(self) -> Result<PaymentVoucher, BuildError> {
        Ok(PaymentVoucher {
            amount: self.amount,
            barcode: self.barcode,
            barcode_format: self.barcode_format,
            company_name: self.company_name,
            document_url: self.document_url,
            expires_at: self.expires_at,
            provider_logo: self.provider_logo,
            provider_name: self.provider_name,
            reference: self.reference,
        })
    }
}
