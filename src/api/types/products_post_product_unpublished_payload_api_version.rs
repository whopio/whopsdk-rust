pub use crate::prelude::*;

/// The API version for this webhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostProductUnpublishedPayloadApiVersion {
    #[serde(rename = "v1")]
    V1,
}
impl fmt::Display for PostProductUnpublishedPayloadApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1 => "v1",
        };
        write!(f, "{}", s)
    }
}
