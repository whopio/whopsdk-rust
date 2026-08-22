pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostInvoiceCreatedPayloadType {
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
}
impl fmt::Display for PostInvoiceCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvoiceCreated => "invoice.created",
        };
        write!(f, "{}", s)
    }
}
