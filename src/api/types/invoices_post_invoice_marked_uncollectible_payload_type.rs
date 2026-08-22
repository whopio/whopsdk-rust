pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostInvoiceMarkedUncollectiblePayloadType {
    #[serde(rename = "invoice.marked_uncollectible")]
    InvoiceMarkedUncollectible,
}
impl fmt::Display for PostInvoiceMarkedUncollectiblePayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::InvoiceMarkedUncollectible => "invoice.marked_uncollectible",
        };
        write!(f, "{}", s)
    }
}
