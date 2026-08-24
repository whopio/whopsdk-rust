pub use crate::prelude::*;

/// The method of collection for an invoice.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceCollectionMethods {
    SendInvoice,
    ChargeAutomatically,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceCollectionMethods {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SendInvoice => serializer.serialize_str("send_invoice"),
            Self::ChargeAutomatically => serializer.serialize_str("charge_automatically"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceCollectionMethods {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "send_invoice" => Ok(Self::SendInvoice),
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceCollectionMethods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendInvoice => write!(f, "send_invoice"),
            Self::ChargeAutomatically => write!(f, "charge_automatically"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
