pub use crate::prelude::*;

/// How renewals are collected. `charge_automatically` charges the saved payment method each period; `send_invoice` emails the buyer an invoice each period. `null` when not set.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipCollectionMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ChargeAutomatically => serializer.serialize_str("charge_automatically"),
            Self::SendInvoice => serializer.serialize_str("send_invoice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            "send_invoice" => Ok(Self::SendInvoice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipCollectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChargeAutomatically => write!(f, "charge_automatically"),
            Self::SendInvoice => write!(f, "send_invoice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
