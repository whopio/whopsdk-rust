pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentQr {
    /// Exactly what the buyer must pay, in the charged currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// A hosted page with the complete, printable instructions. If you would rather not render the details yourself, send the buyer here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_url: Option<String>,
    /// When the code stops being payable, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// An account key the buyer can pay to directly (Colombia's Bre-B llave), for apps that take a key instead of a scan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The QR code's contents, ready to render as a scannable image — `qr_format` says how it is encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_code: Option<String>,
    /// How `qr_code` is encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_format: Option<String>,
}

impl PaymentQr {
    pub fn builder() -> PaymentQrBuilder {
        <PaymentQrBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentQrBuilder {
    amount: Option<Money>,
    document_url: Option<String>,
    expires_at: Option<String>,
    key: Option<String>,
    qr_code: Option<String>,
    qr_format: Option<String>,
}

impl PaymentQrBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
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

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn qr_code(mut self, value: impl Into<String>) -> Self {
        self.qr_code = Some(value.into());
        self
    }

    pub fn qr_format(mut self, value: impl Into<String>) -> Self {
        self.qr_format = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentQr`].
    pub fn build(self) -> Result<PaymentQr, BuildError> {
        Ok(PaymentQr {
            amount: self.amount,
            document_url: self.document_url,
            expires_at: self.expires_at,
            key: self.key,
            qr_code: self.qr_code,
            qr_format: self.qr_format,
        })
    }
}
