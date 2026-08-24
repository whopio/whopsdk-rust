pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductUpdatedPayloadType {
    #[serde(rename = "product.updated")]
    ProductUpdated,
}
impl fmt::Display for PostProductUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ProductUpdated => "product.updated",
        };
        write!(f, "{}", s)
    }
}
