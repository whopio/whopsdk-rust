pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostInvoicePaidPayloadType {
    #[serde(rename = "invoice.paid")]
    InvoicePaid,
}
impl fmt::Display for PostInvoicePaidPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvoicePaid => "invoice.paid",
        };
        write!(f, "{}", s)
    }
}
