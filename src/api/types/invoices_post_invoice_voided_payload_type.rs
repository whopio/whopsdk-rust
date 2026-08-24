pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostInvoiceVoidedPayloadType {
    #[serde(rename = "invoice.voided")]
    InvoiceVoided,
}
impl fmt::Display for PostInvoiceVoidedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvoiceVoided => "invoice.voided",
        };
        write!(f, "{}", s)
    }
}
