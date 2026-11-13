pub use crate::prelude::*;

/// The `collection_method` a draft's scheduled send applies when it fires. `null` when the draft has no scheduled send or it was scheduled without one, and once the invoice is issued.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceScheduledCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceScheduledCollectionMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ChargeAutomatically => serializer.serialize_str("charge_automatically"),
            Self::SendInvoice => serializer.serialize_str("send_invoice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceScheduledCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            "send_invoice" => Ok(Self::SendInvoice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceScheduledCollectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChargeAutomatically => write!(f, "charge_automatically"),
            Self::SendInvoice => write!(f, "send_invoice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
