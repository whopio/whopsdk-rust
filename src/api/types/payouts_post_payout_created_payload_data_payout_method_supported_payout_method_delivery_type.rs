pub use crate::prelude::*;

/// How the funds are delivered to the recipient.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType {
    CashPickup,
    BankDeposit,
    HomeDelivery,
    MobileWallet,
    Card,
    Check,
    Bill,
    Cryptocurrency,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CashPickup => serializer.serialize_str("cash_pickup"),
            Self::BankDeposit => serializer.serialize_str("bank_deposit"),
            Self::HomeDelivery => serializer.serialize_str("home_delivery"),
            Self::MobileWallet => serializer.serialize_str("mobile_wallet"),
            Self::Card => serializer.serialize_str("card"),
            Self::Check => serializer.serialize_str("check"),
            Self::Bill => serializer.serialize_str("bill"),
            Self::Cryptocurrency => serializer.serialize_str("cryptocurrency"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cash_pickup" => Ok(Self::CashPickup),
            "bank_deposit" => Ok(Self::BankDeposit),
            "home_delivery" => Ok(Self::HomeDelivery),
            "mobile_wallet" => Ok(Self::MobileWallet),
            "card" => Ok(Self::Card),
            "check" => Ok(Self::Check),
            "bill" => Ok(Self::Bill),
            "cryptocurrency" => Ok(Self::Cryptocurrency),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PostPayoutCreatedPayloadDataPayoutMethodSupportedPayoutMethodDeliveryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CashPickup => write!(f, "cash_pickup"),
            Self::BankDeposit => write!(f, "bank_deposit"),
            Self::HomeDelivery => write!(f, "home_delivery"),
            Self::MobileWallet => write!(f, "mobile_wallet"),
            Self::Card => write!(f, "card"),
            Self::Check => write!(f, "check"),
            Self::Bill => write!(f, "bill"),
            Self::Cryptocurrency => write!(f, "cryptocurrency"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
