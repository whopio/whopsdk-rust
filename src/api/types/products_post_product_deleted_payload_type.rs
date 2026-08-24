pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductDeletedPayloadType {
    #[serde(rename = "product.deleted")]
    ProductDeleted,
}
impl fmt::Display for PostProductDeletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProductDeleted => "product.deleted",
        };
        write!(f, "{}", s)
    }
}
