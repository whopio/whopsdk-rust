pub use crate::prelude::*;

/// The family the type belongs to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentMethodDisplayCategory {
    Card,
    Wallet,
    BankDebit,
    BankTransfer,
    Voucher,
    Redirect,
    Crypto,
    Balance,
    InAppPurchase,
    Saved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentMethodDisplayCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Card => serializer.serialize_str("card"),
            Self::Wallet => serializer.serialize_str("wallet"),
            Self::BankDebit => serializer.serialize_str("bank_debit"),
            Self::BankTransfer => serializer.serialize_str("bank_transfer"),
            Self::Voucher => serializer.serialize_str("voucher"),
            Self::Redirect => serializer.serialize_str("redirect"),
            Self::Crypto => serializer.serialize_str("crypto"),
            Self::Balance => serializer.serialize_str("balance"),
            Self::InAppPurchase => serializer.serialize_str("in_app_purchase"),
            Self::Saved => serializer.serialize_str("saved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentMethodDisplayCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card" => Ok(Self::Card),
            "wallet" => Ok(Self::Wallet),
            "bank_debit" => Ok(Self::BankDebit),
            "bank_transfer" => Ok(Self::BankTransfer),
            "voucher" => Ok(Self::Voucher),
            "redirect" => Ok(Self::Redirect),
            "crypto" => Ok(Self::Crypto),
            "balance" => Ok(Self::Balance),
            "in_app_purchase" => Ok(Self::InAppPurchase),
            "saved" => Ok(Self::Saved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentMethodDisplayCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Card => write!(f, "card"),
            Self::Wallet => write!(f, "wallet"),
            Self::BankDebit => write!(f, "bank_debit"),
            Self::BankTransfer => write!(f, "bank_transfer"),
            Self::Voucher => write!(f, "voucher"),
            Self::Redirect => write!(f, "redirect"),
            Self::Crypto => write!(f, "crypto"),
            Self::Balance => write!(f, "balance"),
            Self::InAppPurchase => write!(f, "in_app_purchase"),
            Self::Saved => write!(f, "saved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
