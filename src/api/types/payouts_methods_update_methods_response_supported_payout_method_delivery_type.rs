pub use crate::prelude::*;

/// How funds are delivered.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateMethodsResponseSupportedPayoutMethodDeliveryType {
    CashPickup,
    BankDeposit,
    HomeDelivery,
    MobileWallet,
    MasspayCard,
    PaperCheck,
    Bill,
    Cryptocurrency,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateMethodsResponseSupportedPayoutMethodDeliveryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CashPickup => serializer.serialize_str("cash_pickup"),
            Self::BankDeposit => serializer.serialize_str("bank_deposit"),
            Self::HomeDelivery => serializer.serialize_str("home_delivery"),
            Self::MobileWallet => serializer.serialize_str("mobile_wallet"),
            Self::MasspayCard => serializer.serialize_str("masspay_card"),
            Self::PaperCheck => serializer.serialize_str("paper_check"),
            Self::Bill => serializer.serialize_str("bill"),
            Self::Cryptocurrency => serializer.serialize_str("cryptocurrency"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateMethodsResponseSupportedPayoutMethodDeliveryType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cash_pickup" => Ok(Self::CashPickup),
            "bank_deposit" => Ok(Self::BankDeposit),
            "home_delivery" => Ok(Self::HomeDelivery),
            "mobile_wallet" => Ok(Self::MobileWallet),
            "masspay_card" => Ok(Self::MasspayCard),
            "paper_check" => Ok(Self::PaperCheck),
            "bill" => Ok(Self::Bill),
            "cryptocurrency" => Ok(Self::Cryptocurrency),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateMethodsResponseSupportedPayoutMethodDeliveryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CashPickup => write!(f, "cash_pickup"),
            Self::BankDeposit => write!(f, "bank_deposit"),
            Self::HomeDelivery => write!(f, "home_delivery"),
            Self::MobileWallet => write!(f, "mobile_wallet"),
            Self::MasspayCard => write!(f, "masspay_card"),
            Self::PaperCheck => write!(f, "paper_check"),
            Self::Bill => write!(f, "bill"),
            Self::Cryptocurrency => write!(f, "cryptocurrency"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
