pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAdUpdatedPayloadType {
    #[serde(rename = "ad.updated")]
    AdUpdated,
}
impl fmt::Display for PostAdUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AdUpdated => "ad.updated",
        };
        write!(f, "{}", s)
    }
}
