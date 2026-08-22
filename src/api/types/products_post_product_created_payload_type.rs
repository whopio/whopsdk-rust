pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductCreatedPayloadType {
    #[serde(rename = "product.created")]
    ProductCreated,
}
impl fmt::Display for PostProductCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProductCreated => "product.created",
        };
        write!(f, "{}", s)
    }
}
