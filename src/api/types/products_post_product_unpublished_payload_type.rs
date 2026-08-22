pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductUnpublishedPayloadType {
    #[serde(rename = "product.unpublished")]
    ProductUnpublished,
}
impl fmt::Display for PostProductUnpublishedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProductUnpublished => "product.unpublished",
        };
        write!(f, "{}", s)
    }
}
