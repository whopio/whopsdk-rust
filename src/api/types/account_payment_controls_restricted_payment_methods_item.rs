pub use crate::prelude::*;

/// Card brands blocked at checkout for this account. Empty when none are blocked. The account cannot re-enable them itself.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountPaymentControlsRestrictedPaymentMethodsItem {
    CardVisa,
    CardMastercard,
    CardAmericanExpress,
    CardDiscoverGlobalNetwork,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountPaymentControlsRestrictedPaymentMethodsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CardVisa => serializer.serialize_str("card_visa"),
            Self::CardMastercard => serializer.serialize_str("card_mastercard"),
            Self::CardAmericanExpress => serializer.serialize_str("card_american_express"),
            Self::CardDiscoverGlobalNetwork => {
                serializer.serialize_str("card_discover_global_network")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountPaymentControlsRestrictedPaymentMethodsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "card_visa" => Ok(Self::CardVisa),
            "card_mastercard" => Ok(Self::CardMastercard),
            "card_american_express" => Ok(Self::CardAmericanExpress),
            "card_discover_global_network" => Ok(Self::CardDiscoverGlobalNetwork),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountPaymentControlsRestrictedPaymentMethodsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CardVisa => write!(f, "card_visa"),
            Self::CardMastercard => write!(f, "card_mastercard"),
            Self::CardAmericanExpress => write!(f, "card_american_express"),
            Self::CardDiscoverGlobalNetwork => write!(f, "card_discover_global_network"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
