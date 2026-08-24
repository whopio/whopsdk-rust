pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostInvoicePastDuePayloadType {
    #[serde(rename = "invoice.past_due")]
    InvoicePastDue,
}
impl fmt::Display for PostInvoicePastDuePayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvoicePastDue => "invoice.past_due",
        };
        write!(f, "{}", s)
    }
}
