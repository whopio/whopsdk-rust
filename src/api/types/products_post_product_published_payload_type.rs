pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductPublishedPayloadType {
    #[serde(rename = "product.published")]
    ProductPublished,
}
impl fmt::Display for PostProductPublishedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProductPublished => "product.published",
        };
        write!(f, "{}", s)
    }
}
