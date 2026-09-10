pub use crate::prelude::*;

/// The types of fee markups that can be configured
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeeMarkupTypes {
    CryptoWithdrawalMarkup,
    RtpWithdrawalMarkup,
    NextDayBankWithdrawalMarkup,
    BankWireWithdrawalMarkup,
    DigitalWalletWithdrawalMarkup,
    TransferMarkup,
    CryptoDepositMarkup,
    BankDepositMarkup,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FeeMarkupTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CryptoWithdrawalMarkup => serializer.serialize_str("crypto_withdrawal_markup"),
            Self::RtpWithdrawalMarkup => serializer.serialize_str("rtp_withdrawal_markup"),
            Self::NextDayBankWithdrawalMarkup => {
                serializer.serialize_str("next_day_bank_withdrawal_markup")
            }
            Self::BankWireWithdrawalMarkup => {
                serializer.serialize_str("bank_wire_withdrawal_markup")
            }
            Self::DigitalWalletWithdrawalMarkup => {
                serializer.serialize_str("digital_wallet_withdrawal_markup")
            }
            Self::TransferMarkup => serializer.serialize_str("transfer_markup"),
            Self::CryptoDepositMarkup => serializer.serialize_str("crypto_deposit_markup"),
            Self::BankDepositMarkup => serializer.serialize_str("bank_deposit_markup"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FeeMarkupTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "crypto_withdrawal_markup" => Ok(Self::CryptoWithdrawalMarkup),
            "rtp_withdrawal_markup" => Ok(Self::RtpWithdrawalMarkup),
            "next_day_bank_withdrawal_markup" => Ok(Self::NextDayBankWithdrawalMarkup),
            "bank_wire_withdrawal_markup" => Ok(Self::BankWireWithdrawalMarkup),
            "digital_wallet_withdrawal_markup" => Ok(Self::DigitalWalletWithdrawalMarkup),
            "transfer_markup" => Ok(Self::TransferMarkup),
            "crypto_deposit_markup" => Ok(Self::CryptoDepositMarkup),
            "bank_deposit_markup" => Ok(Self::BankDepositMarkup),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FeeMarkupTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CryptoWithdrawalMarkup => write!(f, "crypto_withdrawal_markup"),
            Self::RtpWithdrawalMarkup => write!(f, "rtp_withdrawal_markup"),
            Self::NextDayBankWithdrawalMarkup => write!(f, "next_day_bank_withdrawal_markup"),
            Self::BankWireWithdrawalMarkup => write!(f, "bank_wire_withdrawal_markup"),
            Self::DigitalWalletWithdrawalMarkup => write!(f, "digital_wallet_withdrawal_markup"),
            Self::TransferMarkup => write!(f, "transfer_markup"),
            Self::CryptoDepositMarkup => write!(f, "crypto_deposit_markup"),
            Self::BankDepositMarkup => write!(f, "bank_deposit_markup"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
