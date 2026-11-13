pub use crate::prelude::*;

/// How the subscription this invoice starts collects each renewal. `charge_automatically` charges the payment method that paid the invoice each period: the draft's `payment_method_id`, or the one the customer pays the invoice link with. `send_invoice` issues a numbered invoice each period, due on the renewal date and emailed with its payment link to the latest invoice's `recipient_emails`, so it needs at least one recipient and isn't available for invoices billed to a business. Defaults to `charge_automatically`. Only for a draft with a recurring item. A scheduled send applies it when it fires (see `scheduled_collection_method`).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueInvoicesRequestCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IssueInvoicesRequestCollectionMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ChargeAutomatically => serializer.serialize_str("charge_automatically"),
            Self::SendInvoice => serializer.serialize_str("send_invoice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IssueInvoicesRequestCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            "send_invoice" => Ok(Self::SendInvoice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IssueInvoicesRequestCollectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChargeAutomatically => write!(f, "charge_automatically"),
            Self::SendInvoice => write!(f, "send_invoice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
