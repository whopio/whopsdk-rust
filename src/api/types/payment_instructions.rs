pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum PaymentInstructions {
    #[serde(rename = "bank_transfer")]
    #[non_exhaustive]
    BankTransfer {
        #[serde(default)]
        bank_transfer: PaymentBankTransfer,
    },

    #[serde(rename = "qr")]
    #[non_exhaustive]
    Qr {
        #[serde(default)]
        qr: PaymentQr,
    },

    #[serde(rename = "voucher")]
    #[non_exhaustive]
    Voucher {
        #[serde(default)]
        voucher: PaymentVoucher,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PaymentInstructions {
    pub fn bank_transfer(bank_transfer: PaymentBankTransfer) -> Self {
        Self::BankTransfer { bank_transfer }
    }

    pub fn qr(qr: PaymentQr) -> Self {
        Self::Qr { qr }
    }

    pub fn voucher(voucher: PaymentVoucher) -> Self {
        Self::Voucher { voucher }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
